//! OSC 7 escape sequence scanner.
//!
//! Parses `ESC ] 7 ; file://HOST/PATH BEL` (or `ESC \\` terminator) out of a
//! byte stream, exposing the decoded `PATH` to the caller and yielding a
//! cleaned stream with those sequences removed so they never reach the
//! terminal emulator.

const ESC: u8 = 0x1B;
const BEL: u8 = 0x07;
const BACKSLASH: u8 = b'\\';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    Esc,
    OscStart,
    OscSeven,
    OscPayload,
    OscPayloadEsc,
}

pub struct Osc7Scanner {
    state: State,
    payload: Vec<u8>,
    pending: Vec<u8>,
}

impl Osc7Scanner {
    pub fn new() -> Self {
        Self {
            state: State::Normal,
            payload: Vec::new(),
            pending: Vec::new(),
        }
    }

    /// Feed a chunk of bytes. Returns `(cleaned_bytes, extracted_path)`.
    /// `extracted_path` is `Some` when at least one complete OSC 7 sequence
    /// was consumed during this call — if multiple were present, only the
    /// last one is reported (it reflects the current cwd).
    pub fn feed(&mut self, buf: &[u8]) -> (Vec<u8>, Option<String>) {
        let mut out = Vec::with_capacity(buf.len());
        let mut extracted: Option<String> = None;

        for &b in buf {
            match self.state {
                State::Normal => {
                    if b == ESC {
                        self.state = State::Esc;
                        self.pending.clear();
                        self.pending.push(b);
                    } else {
                        out.push(b);
                    }
                }
                State::Esc => {
                    self.pending.push(b);
                    if b == b']' {
                        self.state = State::OscStart;
                    } else {
                        // Not an OSC — flush pending as literal.
                        out.extend_from_slice(&self.pending);
                        self.pending.clear();
                        self.state = State::Normal;
                    }
                }
                State::OscStart => {
                    self.pending.push(b);
                    if b == b'7' {
                        self.state = State::OscSeven;
                    } else {
                        out.extend_from_slice(&self.pending);
                        self.pending.clear();
                        self.state = State::Normal;
                    }
                }
                State::OscSeven => {
                    self.pending.push(b);
                    if b == b';' {
                        self.state = State::OscPayload;
                        self.payload.clear();
                    } else {
                        out.extend_from_slice(&self.pending);
                        self.pending.clear();
                        self.state = State::Normal;
                    }
                }
                State::OscPayload => {
                    if b == BEL {
                        extracted = Some(decode_osc7_payload(&self.payload));
                        self.payload.clear();
                        self.pending.clear();
                        self.state = State::Normal;
                    } else if b == ESC {
                        self.state = State::OscPayloadEsc;
                    } else {
                        self.payload.push(b);
                    }
                }
                State::OscPayloadEsc => {
                    if b == BACKSLASH {
                        extracted = Some(decode_osc7_payload(&self.payload));
                        self.payload.clear();
                        self.pending.clear();
                        self.state = State::Normal;
                    } else {
                        // Unexpected — treat as literal ESC in payload.
                        self.payload.push(ESC);
                        self.payload.push(b);
                        self.state = State::OscPayload;
                    }
                }
            }
        }

        (out, extracted)
    }
}

/// Decode a raw OSC 7 payload (`file://host/path` form) into a filesystem
/// path. Strips the `file://HOST` prefix and percent-decodes the remainder.
fn decode_osc7_payload(raw: &[u8]) -> String {
    let s = String::from_utf8_lossy(raw);
    let without_scheme = s.strip_prefix("file://").unwrap_or(&s);
    // The part before the first `/` is the hostname (may be empty).
    let path = without_scheme
        .find('/')
        .map_or(without_scheme, |idx| &without_scheme[idx..]);
    percent_decode(path)
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2]))
        {
            out.push((h << 4) | l);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seq(host: &str, path: &str) -> Vec<u8> {
        let mut v = vec![ESC, b']', b'7', b';'];
        v.extend_from_slice(b"file://");
        v.extend_from_slice(host.as_bytes());
        v.extend_from_slice(path.as_bytes());
        v.push(BEL);
        v
    }

    #[test]
    fn passes_through_normal_bytes() {
        let mut s = Osc7Scanner::new();
        let (out, cwd) = s.feed(b"hello world\n");
        assert_eq!(out, b"hello world\n");
        assert!(cwd.is_none());
    }

    #[test]
    fn extracts_simple_sequence() {
        let mut s = Osc7Scanner::new();
        let mut buf = b"prefix".to_vec();
        buf.extend(seq("host", "/home/user"));
        buf.extend_from_slice(b"suffix");
        let (out, cwd) = s.feed(&buf);
        assert_eq!(out, b"prefixsuffix");
        assert_eq!(cwd.as_deref(), Some("/home/user"));
    }

    #[test]
    fn percent_decodes_spaces() {
        let mut s = Osc7Scanner::new();
        let (_, cwd) = s.feed(&seq("h", "/tmp/with%20space"));
        assert_eq!(cwd.as_deref(), Some("/tmp/with space"));
    }

    #[test]
    fn handles_st_terminator() {
        let mut s = Osc7Scanner::new();
        let mut buf = vec![ESC, b']', b'7', b';'];
        buf.extend_from_slice(b"file:///var");
        buf.push(ESC);
        buf.push(b'\\');
        let (out, cwd) = s.feed(&buf);
        assert!(out.is_empty());
        assert_eq!(cwd.as_deref(), Some("/var"));
    }

    #[test]
    fn empty_host() {
        let mut s = Osc7Scanner::new();
        let (_, cwd) = s.feed(&seq("", "/etc"));
        assert_eq!(cwd.as_deref(), Some("/etc"));
    }

    #[test]
    fn straddled_buffer() {
        let mut s = Osc7Scanner::new();
        let full = seq("h", "/usr/local");
        let (a, b) = full.split_at(6);
        let (out1, cwd1) = s.feed(a);
        assert!(out1.is_empty());
        assert!(cwd1.is_none());
        let (out2, cwd2) = s.feed(b);
        assert!(out2.is_empty());
        assert_eq!(cwd2.as_deref(), Some("/usr/local"));
    }

    #[test]
    fn multiple_sequences_returns_last() {
        let mut s = Osc7Scanner::new();
        let mut buf = seq("h", "/one");
        buf.extend(seq("h", "/two"));
        let (out, cwd) = s.feed(&buf);
        assert!(out.is_empty());
        assert_eq!(cwd.as_deref(), Some("/two"));
    }

    #[test]
    fn malformed_esc_is_passed_through() {
        let mut s = Osc7Scanner::new();
        let (out, cwd) = s.feed(&[ESC, b'A', b'B']);
        assert_eq!(out, &[ESC, b'A', b'B']);
        assert!(cwd.is_none());
    }

    #[test]
    fn non_osc7_sequence_passes_through() {
        let mut s = Osc7Scanner::new();
        let buf = [ESC, b']', b'0', b';', b'x', BEL];
        let (out, cwd) = s.feed(&buf);
        assert_eq!(out, buf);
        assert!(cwd.is_none());
    }
}
