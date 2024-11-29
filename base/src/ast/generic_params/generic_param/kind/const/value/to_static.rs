use crate::ast::ConstParamValue;
use std::borrow::Cow;

impl<'a> ConstParamValue<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ConstParamValue<'static> {
        match self {
            ConstParamValue::Block(block) => ConstParamValue::Block(block.into_static()),
            ConstParamValue::Identifier(identifier) => {
                ConstParamValue::Identifier(Cow::Owned(match identifier {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }))
            }
            ConstParamValue::Literal(minus, literal) => ConstParamValue::Literal(
                minus,
                Cow::Owned(match literal {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }),
            ),
        }
    }
}
