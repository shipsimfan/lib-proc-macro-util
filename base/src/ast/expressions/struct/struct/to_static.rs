use crate::ast::expressions::{StructBase, StructExprStruct};

impl<'a> StructExprStruct<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructExprStruct<'static> {
        StructExprStruct {
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(comma, field)| (comma, field.into_static()))
                .collect(),
            base: self
                .base
                .map(|(comma, base)| (comma, base.map(StructBase::into_static))),
        }
    }
}
