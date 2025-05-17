use crate::ast::expressions::StructExprFieldName;
use std::borrow::Cow;

impl<'a> StructExprFieldName<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructExprFieldName<'static> {
        match self {
            StructExprFieldName::Identifier(identifier) => {
                StructExprFieldName::Identifier(Cow::Owned(match identifier {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }))
            }
            StructExprFieldName::Tuple(literal) => {
                StructExprFieldName::Tuple(Cow::Owned(match literal {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }))
            }
        }
    }
}
