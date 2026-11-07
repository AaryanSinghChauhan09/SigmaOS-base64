//! Codec model.

use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone)]
pub enum DecodeResult {
    Success(Vec<u8>),
    InvalidInput,
}

#[derive(Debug, Clone)]
pub struct Base64Codec {
    pub wrap: bool,
}

impl Default for Base64Codec {
    fn default() -> Self {
        Self { wrap: false }
    }
}

impl Base64Codec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn encode(&self, data: &[u8]) -> String {
        if self.wrap {
            general_purpose::STANDARD.encode(data)
        } else {
            general_purpose::STANDARD_NO_PAD.encode(data)
        }
    }

    pub fn decode(&self, encoded: &str) -> DecodeResult {
        let engine = general_purpose::STANDARD;
        match engine.decode(encoded) {
            Ok(data) => DecodeResult::Success(data),
            Err(_) => DecodeResult::InvalidInput,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_codec() {
        let c = Base64Codec::new();
        let e = c.encode(b"hello");
        assert!(!e.is_empty());
    }
}
