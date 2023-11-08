use crate::{
    ast::{Lifetime, Punctuated},
    tokens::{Colon, Plus},
};

#[derive(Clone)]
pub struct PredicateLifetime {
    pub lifetime: Lifetime,
    pub colon: Colon,
    pub bounds: Punctuated<Lifetime, Plus>,
}
