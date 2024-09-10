use crate::{ErrorMessage, Span};

impl ErrorMessage {
    /// Creates a new [`ErrorMessage`] at `Span::call_site()`
    ///
    /// ## Parameters
    ///  * `message` - The message which will be displayed
    ///
    /// ## Return Value
    /// Returns the newly created [`ErrorMessage`]
    pub fn new<T: std::fmt::Display>(message: T) -> Self {
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
    pub fn new_at<T: std::fmt::Display>(message: T, span: Span) -> Self {
        ErrorMessage {
            span,
            message: message.to_string(),
        }
    }
}
