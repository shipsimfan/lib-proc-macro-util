use crate::{
    tokens::{IntoLiteral, Literal},
    Span,
};

impl Literal {
    /// Creates a new [`Literal`] from a [`proc_macro::Literal`]
    pub fn new_raw(literal: proc_macro::Literal) -> Self {
        Literal(literal)
    }

    /// Creates a new [`Literal`] at `span`
    pub fn new_at<T: IntoLiteral>(value: T, span: Span) -> Self {
        Literal::new_raw(value.into_literal(span))
    }

    /// Creates a new [`Literal`] at [`Span::call_site`]
    pub fn new<T: IntoLiteral>(value: T) -> Self {
        Literal::new_at(value, Span::call_site())
    }
}
