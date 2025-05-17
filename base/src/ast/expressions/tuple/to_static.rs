use crate::ast::expressions::TupleExpression;

impl<'a> TupleExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TupleExpression<'static> {
        TupleExpression {
            first: self
                .first
                .map(|(expression, comma)| (Box::new(expression.into_static()), comma)),
            remaining: self
                .remaining
                .into_iter()
                .map(|(expression, comma)| (expression.into_static(), comma))
                .collect(),
            last: self
                .last
                .map(|expression| Box::new(expression.into_static())),
        }
    }
}
