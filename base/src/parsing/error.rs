use crate::{Generator, ToTokens, Token};
use proc_macro::{Delimiter, Span};
use std::fmt::Display;

/// An result of parsing macro input
pub type Result<T> = std::result::Result<T, Error>;

/// A set of errors while parsing macro input
pub struct Error {
    errors: Vec<ErrorMessage>,
}

/// An error while parsing macro input
pub struct ErrorMessage {
    /// The span where the error occurred
    span: Span,

    /// The message describing the error
    message: String,
}

impl Error {
    /// Creates a new [`Error`] with one message at `Span::call_site()`
    ///
    /// ## Parameters
    ///  * `message` - The message which will be displayed
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    ///
    /// ## Remarks
    /// For more information, see `ErrorMessage::new()`
    pub fn new<T: Display>(message: T) -> Self {
        Error {
            errors: vec![ErrorMessage::new(message)],
        }
    }

    /// Creates a new [`Error`] with one message
    ///
    /// ## Parameters
    ///  * `message` - The message which will be displayed
    ///  * `span` - The span that will be used for this error
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    ///
    /// ## Remarks
    /// For more information, see `ErrorMessage::new_at()`
    pub fn new_at<T: Display>(message: T, span: Span) -> Self {
        Error {
            errors: vec![ErrorMessage::new_at(message, span)],
        }
    }

    /// Creates a new empty [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created empty [`Error`]
    pub fn new_empty() -> Self {
        Error { errors: Vec::new() }
    }

    /// Appends a message to this error at `Span::call_site()`
    ///
    /// ## Parameters
    ///  * `message` - The message to be appended
    ///
    /// ## Return Value
    /// Returns this error with the message appended
    pub fn append<T: Display>(self, message: T) -> Self {
        self.append_message(ErrorMessage::new(message))
    }

    /// Appends a message to this error
    ///
    /// ## Parameters
    ///  * `message` - The message to be appended
    ///  * `span` - The span for the appended error
    ///
    /// ## Return Value
    /// Returns this error with the message appended
    pub fn append_at<T: Display>(self, message: T, span: Span) -> Self {
        self.append_message(ErrorMessage::new_at(message, span))
    }

    /// Appends a message to this error
    ///
    /// ## Parameters
    ///  * `message` - The message to be appended
    ///
    /// ## Return Value
    /// Returns this error with the message appended
    pub fn append_message(mut self, message: ErrorMessage) -> Self {
        self.errors.push(message);
        self
    }
}

impl ErrorMessage {
    /// Creates a new [`ErrorMessage`] at `Span::call_site()`
    ///
    /// ## Parameters
    ///  * `message` - The message which will be displayed
    ///
    /// ## Return Value
    /// Returns the newly created [`ErrorMessage`]
    pub fn new<T: Display>(message: T) -> Self {
        ErrorMessage {
            span: Span::call_site(),
            message: message.to_string(),
        }
    }

    /// Creates a new [`ErrorMessage`]
    ///
    /// ## Parameters
    ///  * `message` - The message which will be displayed
    ///  * `span` - The span that will be used for this error
    ///
    /// ## Return Value
    /// Returns the newly created [`ErrorMessage`]
    pub fn new_at<T: Display>(message: T, span: Span) -> Self {
        ErrorMessage {
            span,
            message: message.to_string(),
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.errors.len() {
            write!(f, "{}", self.errors[i])?;

            if i < self.errors.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.errors.len() {
            write!(f, "{}", self.errors[i])?;

            if i < self.errors.len() - 1 {
                write!(f, ";")?;
            }
        }

        Ok(())
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, generator: &mut Generator) {
        for error in &self.errors {
            error.to_tokens(generator);
        }
    }
}

impl std::error::Error for ErrorMessage {}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::fmt::Debug for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl ToTokens for ErrorMessage {
    fn to_tokens(&self, generator: &mut Generator) {
        let (start, end) = (self.span.start(), self.span.end());

        <Token![::]>::new([start; 2]).to_tokens(generator);
        generator.identifier_string_at("core", start);
        <Token![::]>::new([start; 2]).to_tokens(generator);
        generator.identifier_string_at("compile_error", start);
        <Token![!]>::new([start]).to_tokens(generator);

        let mut group = generator.group_at(Delimiter::Parenthesis, end);
        group.literal_string_at(&self.message, end);

        <Token![;]>::new([end]).to_tokens(generator);
    }
}
