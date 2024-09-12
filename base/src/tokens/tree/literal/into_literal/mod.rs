use crate::Span;

mod number;
mod string;

/// An item which can be converted into a [`proc_macro::Literal`]
pub trait IntoLiteral {
    /// Create a [`proc_macro::Literal`] from this value
    fn into_literal(self, span: Span) -> proc_macro::Literal;
}
