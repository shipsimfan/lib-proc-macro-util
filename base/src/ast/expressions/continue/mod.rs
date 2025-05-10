use crate::{ast::Lifetime, Token};

mod parse;
mod to_static;
mod to_tokens;

/// When `continue` is encountered, the current iteration of the associated loop body is
/// immediately terminated, returning control to the loop head.
#[derive(Debug, Clone)]
pub struct ContinueExpression<'a> {
    /// The continue keyword identifying this expression
    pub r#continue: Token![continue],

    /// The lifetime to contine the loop on
    pub lifetime: Option<Lifetime<'a>>,
}
