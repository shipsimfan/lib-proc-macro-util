use crate::{
    ast::{Expression, Type},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A fixed-size sequence of homogenous values
#[derive(Debug, Clone)]
pub struct ArrayType<'a> {
    /// The type of the contained values
    pub r#type: Box<Type<'a>>,

    /// Separator between type and count
    pub semi: Token![;],

    /// The number of elements in this array
    pub count: Expression<'a>,
}
