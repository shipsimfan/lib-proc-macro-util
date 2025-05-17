use crate::ast::expressions::RangeExpression;

impl<'a> RangeExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> RangeExpression<'static> {
        RangeExpression {
            lower: self.lower.map(|lower| Box::new(lower.into_static())),
            operator: self.operator,
            upper: self.upper.map(|upper| Box::new(upper.into_static())),
        }
    }
}
