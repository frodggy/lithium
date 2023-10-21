use serde::{Serialize, Deserialize};

use crate::key;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(Vec<u8>);

impl Password {
    pub fn new(pass: String, key: &[u8]) -> Self {
        let hashed_pass = key::encrypt(&pass, key);
        Self(hashed_pass)
    }
}
