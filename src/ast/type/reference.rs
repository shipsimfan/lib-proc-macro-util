use crate::{
    ast::Lifetime,
    tokens::{Ampersand, Mut, Type},
};

#[derive(Clone)]
pub struct TypeReference {
    pub ampersand: Ampersand,
    pub lifetime: Option<Lifetime>,
    pub mutability: Option<Mut>,
    pub element: Box<Type>,
}
