use crate::ast::types::TupleType;

impl<'a> TupleType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TupleType<'static> {
        TupleType {
            types: self
                .types
                .into_iter()
                .map(|(r#type, separator)| (r#type.into_static(), separator))
                .collect(),
            last: self.last.map(|r#type| Box::new(r#type.into_static())),
        }
    }
}
