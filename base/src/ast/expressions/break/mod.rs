use crate::{
    ast::{Expression, Lifetime},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// When break is encountered, execution of the associated loop body is immediately terminated
#[derive(Debug, Clone)]
pub struct BreakExpression<'a> {
    /// The break keyword identifying this expression
    pub r#break: Token![break],

    /// The lifetime to break the loop on
    pub lifetime: Option<Lifetime<'a>>,

    /// An expression to break with
    pub expression: Option<Box<Expression<'a>>>,
}
