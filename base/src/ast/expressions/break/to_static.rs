use crate::ast::{expressions::BreakExpression, Lifetime};

impl<'a> BreakExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> BreakExpression<'static> {
        BreakExpression {
            r#break: self.r#break,
            lifetime: self.lifetime.map(Lifetime::into_static),
            expression: self
                .expression
                .map(|expression| Box::new(expression.into_static())),
        }
    }
}
