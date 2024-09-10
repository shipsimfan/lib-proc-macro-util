use crate::{Error, ErrorMessage, Span};

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
    pub fn new<T: std::fmt::Display>(message: T) -> Self {
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
    pub fn new_at<T: std::fmt::Display>(message: T, span: Span) -> Self {
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
}
