use crate::ast::{items::TupleField, OuterAttribute, Visibility};

impl<'a> TupleField<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TupleField<'static> {
        TupleField {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            visibility: self.visibility.map(Visibility::into_static),
            r#type: self.r#type.into_static(),
        }
    }
}
