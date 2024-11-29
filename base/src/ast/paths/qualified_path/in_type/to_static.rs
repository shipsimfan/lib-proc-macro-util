use crate::ast::QualifiedPathInType;

impl<'a> QualifiedPathInType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> QualifiedPathInType<'static> {
        QualifiedPathInType {
            r#type: self.r#type.into_static(),
            segments: self
                .segments
                .into_iter()
                .map(|(separator, segment)| (separator, segment.into_static()))
                .collect(),
        }
    }
}
