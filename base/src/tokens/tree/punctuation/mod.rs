use crate::{Spacing, Span};

mod display;
mod eq;
mod from;
mod into;
mod new;
mod parse;
mod to_tokens;

/// A single character of punctuation
#[derive(Debug, Clone)]
pub struct Punctuation(pub proc_macro::Punct);

impl Punctuation {
    /// Get the [`Span`] of this punctuation
    ///
    /// ## Return Value
    /// Returns this punctuation's [`Span`]
    pub fn span(&self) -> Span {
        self.0.span()
    }

    /// Gets this punctuation as it's [`char`] representation
    ///
    /// ## Return Value
    /// Returns this punctuation as a [`char`]
    pub fn as_char(&self) -> char {
        self.0.as_char()
    }

    /// Get the [`Spacing`] of this punctuation
    ///
    /// ## Return Value
    /// Returns this punctuation's [`Spacing`]
    pub fn spacing(&self) -> Spacing {
        self.0.spacing()
    }
}
