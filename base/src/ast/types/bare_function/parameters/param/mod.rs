use crate::{
    ast::{OuterAttribute, Type},
    Token,
};

mod name;

mod parse;
mod to_tokens;

pub use name::MaybeNamedParamName;

/// A function parameter which may not have a name
#[derive(Debug, Clone)]
pub struct MaybeNamedParam<'a> {
    /// Attributes modifying this parameter
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The name of the parameter
    pub name: Option<(MaybeNamedParamName, Token![:])>,

    /// The type of the parameter
    pub r#type: Box<Type<'a>>,
}
