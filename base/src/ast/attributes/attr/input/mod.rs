use crate::{ast::Expression, tokens::Group, Token};
use std::borrow::Cow;

mod from;
mod new;
mod parse;
mod to_tokens;

/// The value of an [`Attr`](crate::ast::Attr)
#[derive(Debug, Clone)]
pub enum AttrInput<'a> {
    /// The input is made up of an arbitrary delimted token tree
    Group(Cow<'a, Group>),

    /// The input is equal to an expression
    Expression(Token![=], Expression<'a>),
}
