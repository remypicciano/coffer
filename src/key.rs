use rand::RngCore;
use std::fs;


pub fn generate_key() -> [u8; 32] {

    let mut key = [0u8; 32];

    rand::rng()
        .fill_bytes(&mut key);

    key
}


pub fn save_key(
    key: &[u8; 32],
    path: &str,
) -> Result<(), anyhow::Error> {

    fs::write(path, key)?;

    Ok(())
}