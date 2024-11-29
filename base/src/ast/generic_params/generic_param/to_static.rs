use crate::ast::{GenericParam, OuterAttribute};

impl<'a> GenericParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GenericParam<'static> {
        GenericParam {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            kind: self.kind.into_static(),
        }
    }
}
