use crate::{
    ast::{MaybeIdentifier, OuterAttribute, Type},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A function parameter which may not have a name
#[derive(Debug, Clone)]
pub struct MaybeNamedParam<'a> {
    /// Attributes modifying this parameter
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The name of the parameter
    pub name: Option<(MaybeIdentifier<'a>, Token![:])>,

    /// The type of the parameter
    pub r#type: Box<Type<'a>>,
}
