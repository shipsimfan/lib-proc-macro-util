use crate::{tokens::IntoLiteral, Span};

impl IntoLiteral for char {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::character(self);
        literal.set_span(span);
        literal
    }
}
