use crate::ast::{Type, TypeParam};

impl<'a> TypeParam<'a> {
    /// Convert this generic parameter into a generic argument
    pub fn to_type(&self) -> Type<'a> {
        Type::from_ident(self.identifier.clone())
    }
}
