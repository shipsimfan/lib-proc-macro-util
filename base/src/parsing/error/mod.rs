mod append;
mod display;
mod message;
mod new;
mod to_tokens;

pub use message::ErrorMessage;

/// An result of parsing macro input
pub type Result<T> = std::result::Result<T, Error>;

/// A set of errors while parsing macro input
pub struct Error {
    /// The list of errors
    pub errors: Vec<ErrorMessage>,
}

impl std::error::Error for Error {}
