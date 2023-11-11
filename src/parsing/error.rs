use crate::{
    tokens::{DoubleColon, Exclamation},
    Generator, ToTokens,
};
use proc_macro::{Delimiter, Span};
use std::fmt::Display;

/// An result of parsing macro input
pub type Result<T> = std::result::Result<T, Error>;

/// An error while parsing macro input
pub struct Error {
    /// The span where the error occurred
    span: Span,

    /// The message describing the error
    message: String,
}

impl Error {
    /// Creates a new [`Error`] with span `Span::call_site()`
    ///
    /// ## Parameters
    ///  * `message` - The message which will be displayed
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn new<T: Display>(message: T) -> Self {
        Error {
            span: Span::call_site(),
            message: message.to_string(),
        }
    }

    /// Creates a new [`Error`]
    ///
    /// ## Parameters
    ///  * `span` - The span that will be used for this error
    ///  * `message` - The message which will be displayed
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn new_at<T: Display>(span: Span, message: T) -> Self {
        Error {
            span,
            message: message.to_string(),
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, generator: &mut Generator) {
        let (start, end) = (self.span.start(), self.span.end());

        generator.generate(&DoubleColon::new([start; 2]));
        generator.identifier_string_with_span("core", start);
        generator.generate(&DoubleColon::new([start; 2]));
        generator.identifier_string_with_span("compile_error", start);
        generator.generate(&Exclamation::new([start]));

        let mut group = generator.group_span(Delimiter::Brace, end);
        group.literal_string_with_span(&self.message, end);
    }
}
