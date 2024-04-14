use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{rngs::OsRng, RngCore};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct MasterPassword(pub Vec<u8>); // storing the aes key

pub struct EncryptionService;

impl EncryptionService {
    pub fn generate_key() -> Vec<u8> {
        let mut key = vec![0u8; 32]; // aes-256 key length
        OsRng.fill_bytes(&mut key);
        key
    }

    pub fn generate_iv() -> Vec<u8> {
        let mut iv = vec![0u8; 16]; // aes block size for cbc mode
        OsRng.fill_bytes(&mut iv);
        iv
    }

    // encrypts data using aes-cbc
    pub fn encrypt_data(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256Cbc::new_from_slices(key, iv)
            .map_err(|e| e.to_string())?;
        Ok(cipher.encrypt_vec(data))
    }

    // decrypts data using aes-cbc
    pub fn decrypt_data(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256Cbc::new_from_slices(key, iv)
            .map_err(|e| e.to_string())?;
        cipher.decrypt_vec(encrypted_data)
            .map_err(|e| e.to_string())
    }

    pub fn encrypt_master_password(password: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256Cbc::new_from_slices(key, iv)
            .map_err(|e| e.to_string())?;
        Ok(cipher.encrypt_vec(password))
    }
}