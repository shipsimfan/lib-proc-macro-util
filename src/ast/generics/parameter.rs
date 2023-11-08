use crate::ast::{ConstantParameter, LifetimeParameter, TypeParameter};

#[derive(Clone)]
pub enum GenericParameter {
    Lifetime(LifetimeParameter),
    Type(TypeParameter),
    Constant(ConstantParameter),
}
