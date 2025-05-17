use crate::ast::expressions::StructExpression;

impl<'a> StructExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructExpression<'static> {
        StructExpression {
            path: self.path.into_static(),
            kind: self.kind.into_static(),
        }
    }
}
