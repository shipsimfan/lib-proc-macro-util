use crate::ast::{items::SelfParam, Lifetime, OuterAttribute};

impl<'a> SelfParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> SelfParam<'static> {
        SelfParam {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            reference: self
                .reference
                .map(|(and, lifetime)| (and, lifetime.map(Lifetime::into_static))),
            r#mut: self.r#mut,
            _self: self._self,
            r#type: self
                .r#type
                .map(|(colon, r#type)| (colon, r#type.into_static())),
        }
    }
}
