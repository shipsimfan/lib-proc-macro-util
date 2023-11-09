use crate::{
    ast::{Punctuated, TypeParameterBound},
    tokens::{Impl, Plus},
};

#[derive(Clone)]
pub struct TypeImplTrait {
    pub r#impl: Impl,
    pub bounds: Punctuated<TypeParameterBound, Plus>,
}
