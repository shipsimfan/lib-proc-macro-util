use crate::ast::{types::MaybeNamedParam, OuterAttribute};

impl<'a> MaybeNamedParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MaybeNamedParam<'static> {
        MaybeNamedParam {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            name: self
                .name
                .map(|(identifier, comma)| (identifier.into_static(), comma)),
            r#type: Box::new(self.r#type.into_static()),
        }
    }
}
