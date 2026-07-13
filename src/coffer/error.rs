use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CofferError {
    #[error("This file is not a valid Coffer container.")]
    InvalidContainer,
    #[error("This Coffer file uses an unsupported format version ({0}).")]
    UnsupportedVersion(u8),
    #[error("This Coffer file uses an unsupported encryption algorithm ({0}).")]
    UnsupportedAlgorithm(u8),
    #[error("The selected key file is not a valid Coffer key.")]
    InvalidKey,
    #[error(
        "The file could not be authenticated. The key may not match, or a file may have changed."
    )]
    AuthenticationFailed,
    #[error("The protected filename is not safe to restore.")]
    InvalidFilename,
    #[error("A file already exists at {0}.")]
    OutputExists(PathBuf),
    #[error("Coffer cannot process a file this large on this computer.")]
    FileTooLarge,
    #[error("The operating system could not provide secure random data.")]
    RandomFailed,
    #[error("The operation was cancelled. No incomplete output was kept.")]
    Cancelled,
    #[error("Coffer could not read the selected file.")]
    ReadFailed(#[source] std::io::Error),
    #[error("Coffer could not write the selected destination.")]
    WriteFailed(#[source] std::io::Error),
}

impl CofferError {
    pub fn user_message(&self) -> String {
        self.to_string()
    }

    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::InvalidContainer => "invalid_container",
            Self::UnsupportedVersion(_) => "unsupported_version",
            Self::UnsupportedAlgorithm(_) => "unsupported_algorithm",
            Self::InvalidKey => "invalid_key",
            Self::AuthenticationFailed => "authentication_failed",
            Self::InvalidFilename => "invalid_filename",
            Self::OutputExists(_) => "output_exists",
            Self::FileTooLarge => "file_too_large",
            Self::RandomFailed => "random_failed",
            Self::Cancelled => "cancelled",
            Self::ReadFailed(_) => "read_failed",
            Self::WriteFailed(_) => "write_failed",
        }
    }
}
