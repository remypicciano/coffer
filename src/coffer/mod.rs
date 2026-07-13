mod error;
mod format;
mod key;
mod ops;

pub use error::CofferError;
pub use ops::{
    ProtectRequest, ProtectResult, RestoreRequest, RestoreResult, protect_file, restore_file,
};
