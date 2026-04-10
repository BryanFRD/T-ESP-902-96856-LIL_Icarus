use std::io::Write;
use std::sync::{Arc, Mutex};

use shared::encryption::{decrypt, encrypt};
use shared::error::NetworkError;
use shared::packets::{AgentAuth, Packet, Packets, from_packet_bytes};

use super::{Connection, ReadHalf, WriteHalf};

#[derive(Debug)]
pub struct Client {
    reader: ReadHalf,
    writer: Arc<Mutex<WriteHalf>>,
    shared_secret: [u8; 32],
}

impl Client {
    pub fn new(addr: &str) -> Result<Self, NetworkError> {
        let connection = Connection::connect(addr)?;
        let (mut reader, mut writer) = connection.split();

        let shared_secret = super::perform_handshake(&mut reader, &mut writer)?;

        Ok(Client {
            reader,
            writer: Arc::new(Mutex::new(writer)),
            shared_secret,
        })
    }

    #[allow(dead_code)]
    pub fn shutdown(&self) -> Result<(), NetworkError> {
        let Ok(mut writer) = self.writer.lock() else {
            return Err(NetworkError::LockError);
        };

        writer.write(&[])?;
        writer.flush()?;
        self.writer.lock().unwrap().shutdown()?;
        self.reader.shutdown()?;
        Ok(())
    }

    pub fn send(&self, buf: &[u8]) -> Result<(), NetworkError> {
        let (encrypted_buf, nonce) = encrypt(&self.shared_secret, buf)?;

        #[allow(clippy::cast_possible_truncation)]
        let len = encrypted_buf.len() as u32;
        let total_size = 4 + nonce.len() + encrypted_buf.len();
        let mut data = vec![0u8; total_size];

        data[0..4].copy_from_slice(&len.to_be_bytes());
        data[4..16].copy_from_slice(&nonce);
        data[16..].copy_from_slice(&encrypted_buf);

        let Ok(mut writer) = self.writer.lock() else {
            return Err(NetworkError::LockError);
        };

        writer.write_all(&data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Vec<u8>, NetworkError> {
        let mut len_buf = [0u8; 4];
        self.reader.read_exact(&mut len_buf)?;

        let len = u32::from_be_bytes(len_buf) as usize;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut nonce_buf = vec![0u8; 12];
        self.reader.read_exact(&mut nonce_buf)?;

        let mut encrypted_buf = vec![0u8; len];
        self.reader.read_exact(&mut encrypted_buf)?;

        let decrypted_data = decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf)?;

        Ok(decrypted_data)
    }

    /// Authenticate with the server by sending `audit_id`, token, and hostname.
    /// Returns the agent token and agent ID assigned by the server (if new registration).
    pub fn authenticate(
        &mut self,
        audit_id: &str,
        token: &str,
        hostname: &str,
    ) -> Result<(String, String), NetworkError> {
        let auth_packet =
            AgentAuth::new(audit_id.to_string(), token.to_string(), hostname.to_string());
        let serialized = auth_packet.serialize()?;
        self.send(&serialized)?;

        let response_data = self.receive()?;
        let packet = from_packet_bytes(&response_data)?;

        match packet {
            Packets::AgentAuthResponse(resp) => {
                if resp.success {
                    Ok((resp.agent_token, resp.agent_id))
                } else {
                    Err(NetworkError::AuthenticationFailed(resp.message))
                }
            }
            _ => Err(NetworkError::UnexpectedPacket),
        }
    }

    /// Deconstruct the client into its components (reader, writer, `shared_secret`).
    /// This is useful for the multiplex manager to avoid mutex contention.
    pub fn into_parts(self) -> (ReadHalf, Arc<Mutex<WriteHalf>>, [u8; 32]) {
        (self.reader, self.writer, self.shared_secret)
    }
}
