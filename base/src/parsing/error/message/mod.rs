use crate::Span;

mod display;
mod new;
mod to_tokens;

/// An error while parsing macro input
pub struct ErrorMessage {
    /// The span where the error occurred
    pub span: Span,

    /// The message describing the error
    pub message: String,
}

impl std::error::Error for ErrorMessage {}
