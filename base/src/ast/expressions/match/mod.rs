use crate::{
    ast::{Expression, InnerAttribute},
    Token,
};

mod arm;

mod parse;
mod to_static;
mod to_tokens;

pub use arm::{MatchArm, MatchArmGuard};

/// A match expression branches on a pattern. The exact form of matching that occurs depends on the
/// pattern.
#[derive(Debug, Clone)]
pub struct MatchExpression<'a> {
    /// The match token introducing this statement
    pub r#match: Token![match],

    /// The expression to match
    pub scrutinee: Box<Expression<'a>>,

    /// The attributes effecting all match arms
    pub attributes: Vec<InnerAttribute<'a>>,

    /// The arms that make up this match statement
    pub arms: Vec<(MatchArm<'a>, Token![=>], Expression<'a>, Option<Token![,]>)>,
}
