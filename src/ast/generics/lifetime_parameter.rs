use crate::{
    ast::{Attribute, Lifetime, Punctuated},
    tokens::{Colon, Plus},
};

#[derive(Clone)]
pub struct LifetimeParameter {
    pub attributes: Vec<Attribute>,
    pub lifetime: Lifetime,
    pub colon: Option<Colon>,
    pub bounds: Punctuated<Lifetime, Plus>,
}
