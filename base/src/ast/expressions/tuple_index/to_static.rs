use crate::ast::expressions::TupleIndexExpression;
use std::borrow::Cow;

impl<'a> TupleIndexExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TupleIndexExpression<'static> {
        TupleIndexExpression {
            expression: Box::new(self.expression.into_static()),
            dot: self.dot,
            index: Cow::Owned(match self.index {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
        }
    }
}
