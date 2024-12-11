use crate::ast::{
    expressions::{BlockExpression, LiteralExpression, PathExpression},
    MacroInvocation,
};

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// A specific kind of expression
#[derive(Debug, Clone)]
pub enum ExpressionKind<'a> {
    /// An expression made up of a literal value
    Literal(LiteralExpression<'a>),

    /// An expression made up of only a block
    Block(BlockExpression<'a>),

    /// A path to a type or a variable
    Path(PathExpression<'a>),

    /// The calling of a macro
    MacroInvocation(MacroInvocation<'a>),
}
