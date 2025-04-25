use crate::ast::{
    expressions::{CallExpression, LiteralExpression, OperatorExpression, PathExpression},
    MacroInvocation,
};

mod from;
mod new;
mod parse;
mod to_expression;
mod to_static;
mod to_tokens;

/// A specific type of expression that does not have a block
#[derive(Debug, Clone)]
pub enum ExpressionWithoutBlockKind<'a> {
    /// An expression made up of a literal value
    Literal(LiteralExpression<'a>),

    /// A path to a type or a variable
    Path(PathExpression<'a>),

    /// The calling of a macro
    MacroInvocation(MacroInvocation<'a>),

    /// An expression that contains an operator
    Operator(OperatorExpression<'a>),

    /// A function call
    Call(CallExpression<'a>),
}
