use crate::{ast::expressions::PathExpression, tokens::Literal, Token};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// The bound on a range pattern
#[derive(Debug, Clone)]
pub enum RangePatternBound<'a> {
    /// The bound is a literal
    Literal(Option<Token![-]>, Cow<'a, Literal>),

    /// The bound is a path
    Path(PathExpression<'a>),
}
