use crate::{tokens::IntoLiteral, Span};

impl IntoLiteral for &str {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::string(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for &std::ffi::CStr {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::c_string(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for &[u8] {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::byte_string(self);
        literal.set_span(span);
        literal
    }
}
