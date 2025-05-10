use crate::ast::{expressions::ContinueExpression, Lifetime};

impl<'a> ContinueExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ContinueExpression<'static> {
        ContinueExpression {
            r#continue: self.r#continue,
            lifetime: self.lifetime.map(Lifetime::into_static),
        }
    }
}
