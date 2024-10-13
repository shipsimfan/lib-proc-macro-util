use crate::{
    ast::{OuterAttribute, Type},
    Token,
};

mod name;

pub use name::MaybeNamedParamName;

#[derive(Debug, Clone)]
pub struct MaybeNamedParam<'a> {
    pub attributes: Vec<OuterAttribute<'a>>,
    pub name: Option<(MaybeNamedParamName, Token![:])>,
    pub r#type: Box<Type<'a>>,
}
