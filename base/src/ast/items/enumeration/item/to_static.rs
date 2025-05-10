use crate::ast::{
    items::{EnumItem, EnumItemDiscriminant, EnumItemKind},
    OuterAttribute, Visibility,
};
use std::borrow::Cow;

impl<'a> EnumItem<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> EnumItem<'static> {
        EnumItem {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            visibility: self.visibility.map(Visibility::into_static),
            name: Cow::Owned(match self.name {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
            kind: self.kind.map(EnumItemKind::into_static),
            discriminant: self.discriminant.map(EnumItemDiscriminant::into_static),
        }
    }
}
