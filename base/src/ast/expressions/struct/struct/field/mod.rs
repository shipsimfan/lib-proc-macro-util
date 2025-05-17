use crate::{
    ast::{Expression, OuterAttribute},
    Token,
};

mod name;

mod parse;
mod to_static;
mod to_tokens;

pub use name::StructExprFieldName;

/// A field in a struct expression
#[derive(Debug, Clone)]
pub struct StructExprField<'a> {
    /// The attributes effecting this field
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The name of the field
    pub name: StructExprFieldName<'a>,

    /// The expression the field is equal to
    pub expression: Option<(Token![:], Box<Expression<'a>>)>,
}
