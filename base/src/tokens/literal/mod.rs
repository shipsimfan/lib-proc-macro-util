use crate::Span;

mod display;
mod from;
mod into;
mod into_literal;
mod new;
mod parse;
mod to_tokens;

pub use into_literal::IntoLiteral;

/// A string or numeric literal
#[derive(Debug, Clone)]
pub struct Literal(pub proc_macro::Literal);

impl Literal {
    /// Get the [`Span`] of this literal
    ///
    /// ## Return Value
    /// Returns this literal's [`Span`]
    pub fn span(&self) -> Span {
        self.0.span()
    }
}
