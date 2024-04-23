use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn deserialize_string(string: String) -> String {
    String::from_utf8(STANDARD.decode(string.as_bytes()).unwrap()).unwrap()
}