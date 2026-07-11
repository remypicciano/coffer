use aes_gcm::{
    Aes256Gcm,
    Key,
    Nonce,
    aead::{
        Aead,
        KeyInit,
    },
};

use rand::RngCore;
use std::fs;


pub fn encrypt_file(
    input_path: &str,
    output_path: &str,
    key_bytes: &[u8; 32],
) -> Result<(), anyhow::Error> {

    let plaintext = fs::read(input_path)?;


    let key = Key::<Aes256Gcm>::from_slice(key_bytes);

    let cipher = Aes256Gcm::new(key);


    let mut nonce_bytes = [0u8; 12];

    rand::rng()
        .fill_bytes(&mut nonce_bytes);


    let nonce = Nonce::from_slice(&nonce_bytes);


    let ciphertext = cipher.encrypt(
        nonce,
        plaintext.as_ref()
    )
    .map_err(|_| anyhow::anyhow!("Encryption failed"))?;


    let mut output = Vec::new();

    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);


    fs::write(output_path, output)?;


    Ok(())
}

pub fn decrypt_file(
    input_path: &str,
    key_bytes: &[u8; 32],
) -> Result<String, anyhow::Error> {

    // Read encrypted file
    let encrypted_data = fs::read(input_path)?;


    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);


    // Create AES key
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);


    // Create cipher
    let cipher = Aes256Gcm::new(key);


    // Create nonce
    let nonce = Nonce::from_slice(nonce_bytes);


    // Decrypt
    let plaintext = cipher.decrypt(
        nonce,
        ciphertext
    )
    .map_err(|_| anyhow::anyhow!("Decryption failed"))?;


    // Convert bytes into text
    let plaintext_string = String::from_utf8(plaintext)?;


    Ok(plaintext_string)
}