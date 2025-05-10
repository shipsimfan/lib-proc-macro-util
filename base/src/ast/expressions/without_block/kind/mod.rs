use crate::ast::{
    expressions::{
        BreakExpression, CallExpression, ContinueExpression, FieldExpression, LiteralExpression,
        MethodCallExpression, OperatorExpression, PathExpression, ReturnExpression,
        UnderscoreExpression,
    },
    MacroInvocation,
};

mod from;
mod new;
mod parse;
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

    /// An accessor of a field
    Field(FieldExpression<'a>),

    /// An accessor of a field
    Underscore(UnderscoreExpression),

    /// A call to a method
    MethodCall(MethodCallExpression<'a>),

    /// Continues on to the next iteration of a loop
    Continue(ContinueExpression<'a>),

    /// Stops running a loop
    Break(BreakExpression<'a>),

    /// Stops running a function
    Return(ReturnExpression<'a>),
}
