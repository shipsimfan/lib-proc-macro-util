use crate::ast::patterns::LiteralPattern;
use std::borrow::Cow;

impl<'a> LiteralPattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LiteralPattern<'static> {
        match self {
            LiteralPattern::Literal(neg, literal) => LiteralPattern::Literal(
                neg,
                Cow::Owned(match literal {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                }),
            ),
            LiteralPattern::True(r#true) => LiteralPattern::True(r#true),
            LiteralPattern::False(r#false) => LiteralPattern::False(r#false),
        }
    }
}
