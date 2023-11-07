use proc_macro::Span;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    span: Span,
    message: String,
}

impl Error {
    pub fn new_at<T: std::fmt::Display>(span: Span, message: T) -> Self {
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
