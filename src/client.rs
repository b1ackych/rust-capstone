const SERVER_URL: &str = "127.0.0.1:8080";

// CONSIDERATIONS:
//
// vault-data should be abstract. for now the implementation would be a json file)

pub mod comm {
    use super::SERVER_URL;

    use crate::{vault::*, encryption::EncryptionService as Crypt};
    use base64::prelude::{Engine, BASE64_STANDARD as b64};
    use reqwest::blocking::Client as web;
    use std::error;
    
    impl Vault for DummyVault {
        // READING:
        //
        // 1. get encrypted vault key from the server
        // 2. decrypt it using the master password
        // 3. get encrypted vault-data from the server using REST
        // 4. decrypt the vault-data using the vault-key
        // 
        fn get(&self, password: &str) -> Result<Self, Box<dyn error::Error>> {
            let master_password = password.as_bytes();

            let encrypted_key = fetch_vault_key()?;
            let vault_key = Crypt::decrypt_data(
                &encrypted_key,
                master_password,
                &Crypt::generate_iv()
            )?;

            let encrypted_data = fetch_vault_data()?;
            let decrypted_data = Crypt::decrypt_data(
                &encrypted_data,
                &vault_key,
                &Crypt::generate_iv()
            )?;

            let vault_data: VaultData = serde_json::from_slice(&decrypted_data)?;

            Ok(DummyVault::new(&vault_key, vault_data))
        }
    
        fn push(&self, data: &VaultData, password: &str) -> Result<(), Box<dyn error::Error>> {
            let master_password = password.as_bytes();

            let encrypted_key = fetch_vault_key()?;
            let vault_key = Crypt::decrypt_data(
                &encrypted_key,
                master_password,
                &Crypt::generate_iv()
            )?;
            
            let encrypted_data = Crypt::encrypt_data(
                serde_json::to_vec(data)?.as_slice(),
                &vault_key,
                &Crypt::generate_iv()
            )?;
            
            send_vault_data(&encrypted_data)
        }
    }
    
    impl DummyVault{
        // INITIALIZATION:
        //
        // create a master password
        // generate a vault-key, then encrypt it with the master password
        // send encrypted vault key to the server using rest
        //
        pub fn initialize(&mut self, master_password: &str) { // should throw errors
            let master_password = master_password.as_bytes();

            let _ = Crypt::encrypt_master_password(
                master_password,
                &Crypt::generate_key(),
                &Crypt::generate_iv()
            );

            let vault_key = Crypt::generate_key();
            let _ = send_vault_key(&Crypt::encrypt_data( // the server receives my vault key in ciphertext form
                &vault_key,
                &master_password,
                &Crypt::generate_iv()
            ).unwrap());
            
            self.key = vault_key;
        }
    }
    
    fn fetch_vault_key() -> Result<Vec<u8>, reqwest::Error> {
        web::new().get(format!("{}/vault/key", SERVER_URL)).send()?.bytes().map(|b| b.to_vec())
    }
    fn fetch_vault_data() -> Result<Vec<u8>, reqwest::Error> {
        web::new().get(format!("{}/vault/data", SERVER_URL)).send()?.bytes().map(|b| b.to_vec())
    }

    fn send_vault_key(encrypted_key: &[u8]) -> Result<reqwest::blocking::Response, Box<dyn error::Error>> {
        let encoded_key = b64.encode(encrypted_key);

        let response = web::new()
                            .post(format!("{}/vault/key", SERVER_URL))
                            .body(encoded_key)
                            .header("Content-Type", "application/json")
                            .send()?;
        
        Ok(response)
    }
    fn send_vault_data(encrypted_data: &[u8]) -> Result<(), Box<dyn error::Error>> {
        let encoded_data = b64.encode(encrypted_data);

        let response = web::new()
            .post(format!("{}/vault/data", SERVER_URL))
            .body(encoded_data)
            .header("Content-Type", "application/json")
            .send()?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to send vault data")))
        }
    }
}

// balls