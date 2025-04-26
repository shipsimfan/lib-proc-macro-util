use crate::ast::{items::StructField, OuterAttribute, Visibility};
use std::borrow::Cow;

impl<'a> StructField<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructField<'static> {
        StructField {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            visibility: self.visibility.map(Visibility::into_static),
            name: Cow::Owned(match self.name {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.clone(),
            }),
            colon: self.colon,
            r#type: self.r#type.into_static(),
        }
    }
}
