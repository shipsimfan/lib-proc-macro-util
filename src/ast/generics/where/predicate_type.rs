use crate::{
    ast::{BoundLifetimes, Punctuated, Type, TypeParameterBound},
    tokens::{Colon, Plus},
};

#[derive(Clone)]
pub struct PredicateType {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_type: Type,
    pub colon: Colon,
    pub bounds: Punctuated<TypeParameterBound, Plus>,
}
