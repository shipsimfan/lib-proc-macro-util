use crate::ast::{Item, OuterAttribute};

impl<'a> Item<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Item<'static> {
        Item {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            kind: self.kind.into_static(),
        }
    }
}
