use serde::{Serialize, Deserialize};
use std::error;

pub trait Vault : Sized {
    fn get(&self, password: &str) -> Result<Self, Box<dyn error::Error>>;
    fn push(&self, data: &VaultData, password: &str) -> Result<(), Box<dyn error::Error>>;
}

#[derive(Serialize, Deserialize, Default)]
pub struct VaultData {
    pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub title: String, 
    pub password: String 
}


#[derive(Serialize, Deserialize, Default)]
pub struct DummyVault {
    pub data: VaultData,
    pub key: Vec<u8>
}
impl DummyVault {
    pub fn new(key: &[u8], data: VaultData) -> Self {
        let key = key.to_vec();
        Self { key, data }
    }
}