use crate::ast::{TypeParam, TypeParamBounds};
use std::borrow::Cow;

impl<'a> TypeParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypeParam<'static> {
        TypeParam {
            identifier: Cow::Owned(match self.identifier {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
            bounds: self
                .bounds
                .map(|(colon, bounds)| (colon, bounds.map(TypeParamBounds::into_static))),
            default: self
                .default
                .map(|(eq, r#type)| (eq, Box::new(r#type.into_static()))),
        }
    }
}
