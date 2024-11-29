use crate::ast::QualifiedPathType;

impl<'a> QualifiedPathType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> QualifiedPathType<'static> {
        QualifiedPathType {
            start: self.start,
            r#type: Box::new(self.r#type.into_static()),
            r#as: self
                .r#as
                .map(|(r#as, type_path)| (r#as, type_path.into_static())),
            end: self.end,
        }
    }
}
