use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::Rng;
use std::error;

pub struct Password {
    pub read_key: Vec<u8>,
    pub write_key: Vec<u8>,
    pub hash: Vec<u8>,
}

pub fn generate_password() -> Result<Password, Box<dyn error::Error>> {
    let read_key: Vec<u8>;
    let write_key: Vec<u8>;
    let salt: Vec<u8>;
    {
        let mut rng = rand::thread_rng();
        read_key = rng.gen::<[u8; 8]>().to_vec();
        write_key = rng.gen::<[u8; 20]>().to_vec();
        salt = rng.gen::<[u8; 32]>().to_vec()
    }
    let salt_string = SaltString::encode_b64(salt.as_slice()).map_err(|err| err.to_string())?;
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(write_key.as_slice(), &salt_string)
        .map_err(|err| err.to_string())?
        .to_string();
    Ok(Password {
        read_key,
        write_key,
        hash: hash.as_bytes().to_vec(),
    })
}

pub fn verify_password(write_key: &[u8], hash: &[u8]) -> bool {
    let hash_string = String::from_utf8(hash.to_vec()).unwrap();
    let parsed_hash = PasswordHash::new(&hash_string).unwrap();
    Argon2::default()
        .verify_password(write_key, &parsed_hash)
        .is_ok()
}
