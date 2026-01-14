pub fn redact_string(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= 2 {
        return "x".repeat(chars.len());
    }

    format!("{}xxxx{}", chars[0], chars[chars.len() - 1])
}

pub fn redact_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let (local, domain) = email.split_at(at_pos);
        if local.is_empty() {
            return email.to_string();
        }

        let chars: Vec<char> = local.chars().collect();
        if chars.len() == 1 {
            return format!("x{domain}");
        }

        format!("{}xxxx{}{domain}",chars[0], chars[chars.len() - 1])
    } else {
        redact_string(email)
    }
}

pub fn redact_json_value(value: &mut serde_json::Value, is_admin: bool) {
    let always_hidden_fields = ["password", "token", "secret", "api_key", "jwt"];
    let redact_fields = ["email", "name", "username", "first_name", "last_name", "full_name"];

    match value {
        serde_json::Value::Object(obj) => {
            for (key, val) in obj.iter_mut() {
                if always_hidden_fields.contains(&key.as_str()) {
                    *val = serde_json::Value::String("HIDDEN".to_string());
                } else if !is_admin && redact_fields.contains(&key.as_str()) {
                    if let serde_json::Value::String(s) = val {
                        let redacted = if key == "email" {
                            redact_email(s)
                        } else {
                            redact_string(s)
                        };
                        *val = serde_json::Value::String(redacted);
                    }
                } else {
                    redact_json_value(val, is_admin);
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter_mut() {
                redact_json_value(item, is_admin);
            }
        }
        _ => {}
    }
}
