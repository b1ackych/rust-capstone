use crate::encryption::EncryptionService;
use std::collections::HashMap;

pub struct User {
    email: String,
    encrypted_master_password: Vec<u8>, 
}

pub enum RegistrationError {
    EmailAlreadyExists,
    WeakPassword,
}

pub struct UserManager {
    users: HashMap<String, User>, 
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager {
            users: HashMap::new(),
        }
    }

    pub fn register_user(&mut self, email: &str, master_password: &str) -> Result<(), RegistrationError> {
        if self.users.contains_key(email) {
            return Err(RegistrationError::EmailAlreadyExists);
        }
        
        let key = EncryptionService::generate_key();  
        let iv = EncryptionService::generate_iv();    
        let encrypted_password = EncryptionService::encrypt_master_password(
            master_password.as_bytes(),
            &key,
            &iv
        ).expect("Encryption of master password failed");

        Ok(())
    }
}