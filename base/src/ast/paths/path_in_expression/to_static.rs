use crate::ast::PathInExpression;

impl<'a> PathInExpression<'a> {
    pub fn into_static(self) -> PathInExpression<'static> {
        PathInExpression {
            leading: self.leading,
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(separator, segment)| (separator, segment.into_static()))
                .collect(),
        }
    }
}
