mod error;
pub mod format;
pub mod metadata;
mod utils;

pub use error::AVError;

type Result<T> = std::result::Result<T, AVError>;
