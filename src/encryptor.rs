use sha2::{Digest, Sha256};

pub struct Encryptor {
    secure: Vec<u8>,
}

impl Encryptor {
    pub fn new(secure: &[u8]) -> Self {
        Self {
            secure: secure.to_vec(),
        }
    }

    pub fn encode(&self, raw: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();

        hasher.update(&self.secure);

        hasher.update(raw.as_bytes());

        let result = hasher.finalize();

        result.to_vec()
    }
}
