use crate::{Error, ErrorMessage, Span};

impl Error {
    /// Appends a message to this error at `Span::call_site()`
    ///
    /// ## Parameters
    ///  * `message` - The message to be appended
    ///
    /// ## Return Value
    /// Returns this error with the message appended
    pub fn append<T: std::fmt::Display>(self, message: T) -> Self {
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
    pub fn append_at<T: std::fmt::Display>(self, message: T, span: Span) -> Self {
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
