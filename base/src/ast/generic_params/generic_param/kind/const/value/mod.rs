use std::borrow::Cow;

use crate::{
    ast::expressions::BlockExpression,
    tokens::{Identifier, Literal},
    Token,
};

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// The value of a constant generic parameter
#[derive(Debug, Clone)]
pub enum ConstParamValue<'a> {
    /// The value is contained in a block
    Block(BlockExpression<'a>),

    /// The value is an identifier
    Identifier(Cow<'a, Identifier>),

    /// The value is a literal
    Literal(Option<Token![-]>, Cow<'a, Literal>),
}
