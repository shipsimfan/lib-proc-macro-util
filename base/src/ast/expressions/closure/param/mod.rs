use crate::{
    ast::{OuterAttribute, PatternNoTopAlt, Type},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A parameter to a closure
#[derive(Debug, Clone)]
pub struct ClosureParam<'a> {
    /// The attributes effecting this parameter
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The pattern giving the name of the parameter
    pub pattern: PatternNoTopAlt<'a>,

    /// The type of the parameter
    pub r#type: Option<(Token![:], Box<Type<'a>>)>,
}
