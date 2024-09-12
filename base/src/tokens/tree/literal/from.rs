use crate::tokens::{IntoLiteral, Literal};

impl From<proc_macro::Literal> for Literal {
    fn from(literal: proc_macro::Literal) -> Self {
        Literal::new_raw(literal)
    }
}

impl<T: IntoLiteral> From<T> for Literal {
    fn from(value: T) -> Self {
        Literal::new(value)
    }
}
