use crate::Span;

mod display;
mod eq;
mod from;
mod into;
mod new;
mod parse;
mod to_tokens;

/// An identifier
#[derive(Debug, Clone)]
pub struct Identifier(pub proc_macro::Ident);

impl Identifier {
    /// Get the [`Span`] of this identifier
    ///
    /// ## Return Value
    /// Returns this identifier's [`Span`]
    pub fn span(&self) -> Span {
        self.0.span()
    }

    /// Converts this identifier to a [`String`]
    ///
    /// ## Return Value
    /// Returns the [`String`] representation of this identifier
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
