use crate::ast::QualifiedPathInExpression;

impl<'a> QualifiedPathInExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> QualifiedPathInExpression<'static> {
        QualifiedPathInExpression {
            r#type: self.r#type.into_static(),
            segments: self
                .segments
                .into_iter()
                .map(|(separator, segment)| (separator, segment.into_static()))
                .collect(),
        }
    }
}
