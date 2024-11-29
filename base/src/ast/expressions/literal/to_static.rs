use crate::ast::expressions::LiteralExpression;
use std::borrow::Cow;

impl<'a> LiteralExpression<'a> {
    pub fn into_static(self) -> LiteralExpression<'static> {
        match self {
            LiteralExpression::Literal(literal) => {
                LiteralExpression::Literal(Cow::Owned(match literal {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }))
            }
            LiteralExpression::True(r#true) => LiteralExpression::True(r#true),
            LiteralExpression::False(r#false) => LiteralExpression::False(r#false),
        }
    }
}
