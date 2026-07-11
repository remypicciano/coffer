use std::path::PathBuf;


pub struct CofferPaths {
    pub encrypted_file: Option<PathBuf>,
    pub key_file: Option<PathBuf>,
}


impl CofferPaths {

    pub fn new() -> Self {

        Self {
            encrypted_file: None,
            key_file: None,
        }

    }

}