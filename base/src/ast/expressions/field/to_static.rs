use crate::ast::expressions::FieldExpression;
use std::borrow::Cow;

impl<'a> FieldExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> FieldExpression<'static> {
        FieldExpression {
            expression: Box::new(self.expression.into_static()),
            dot: self.dot,
            identifier: Cow::Owned(match self.identifier {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.clone(),
            }),
        }
    }
}
