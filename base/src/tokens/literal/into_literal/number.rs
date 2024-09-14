use crate::{tokens::IntoLiteral, Span};

impl IntoLiteral for u8 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::u8_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for u16 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::u16_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for u32 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::u32_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for u64 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::u64_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for u128 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::u128_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for usize {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::usize_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for i8 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::i8_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for i16 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::i16_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for i32 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::i32_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for i64 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::i64_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for i128 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::i128_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for isize {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::isize_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for f32 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::f32_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}

impl IntoLiteral for f64 {
    fn into_literal(self, span: Span) -> proc_macro::Literal {
        let mut literal = proc_macro::Literal::f64_unsuffixed(self);
        literal.set_span(span);
        literal
    }
}
