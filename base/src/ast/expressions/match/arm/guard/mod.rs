use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A guard restricting the values to match
#[derive(Debug, Clone)]
pub struct MatchArmGuard<'a> {
    /// The if token introducing this guard
    pub r#if: Token![if],

    /// The expression restricting the values to match
    pub expression: Expression<'a>,
}
