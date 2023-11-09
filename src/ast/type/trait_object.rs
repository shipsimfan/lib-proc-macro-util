use crate::{
    ast::{Punctuated, TypeParameterBound},
    tokens::{Dyn, Plus},
};

#[derive(Clone)]
pub struct TypeTraitObject {
    pub r#dyn: Option<Dyn>,
    pub bounds: Punctuated<TypeParameterBound, Plus>,
}
