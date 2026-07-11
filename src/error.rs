use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CofferError {
    #[error("This file is not a valid Coffer container.")]
    InvalidContainer,

    #[error("This Coffer file uses an unsupported format version ({0}).")]
    UnsupportedVersion(u8),

    #[error("The selected key is not valid.")]
    InvalidKey,

    #[error("The file could not be authenticated. Check that you selected the matching key.")]
    AuthenticationFailed,

    #[error("The protected filename is not safe to restore.")]
    InvalidFilename,

    #[error("A file already exists at {0}.")]
    OutputExists(PathBuf),

    #[error("Coffer could not read {path}.")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Coffer could not write to {path}.")]
    WriteFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl CofferError {
    pub fn user_message(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authentication_error_does_not_distinguish_tampering_from_wrong_key() {
        let message = CofferError::AuthenticationFailed.user_message();
        assert!(message.contains("authenticated"));
        assert!(!message.contains("tamper"));
    }

    #[test]
    fn invalid_container_message_is_user_facing() {
        assert_eq!(
            CofferError::InvalidContainer.user_message(),
            "This file is not a valid Coffer container."
        );
    }
}
