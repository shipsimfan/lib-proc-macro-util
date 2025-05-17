use crate::ast::expressions::StructExprTuple;

impl<'a> StructExprTuple<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructExprTuple<'static> {
        StructExprTuple {
            first: Box::new(self.first.into_static()),
            remaining: self
                .remaining
                .into_iter()
                .map(|(comma, expression)| (comma, expression.into_static()))
                .collect(),
            last: self.last,
        }
    }
}
