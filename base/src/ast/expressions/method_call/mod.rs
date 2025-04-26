use crate::{
    ast::{expressions::CallParams, Expression, PathExprSegment},
    Token,
};

mod to_static;
mod to_tokens;

/// A function call
#[derive(Debug, Clone)]
pub struct MethodCallExpression<'a> {
    /// The source of the method
    pub function: Box<Expression<'a>>,

    /// The dot identifying this method call
    pub dot: Token![.],

    /// The name of the method being called
    pub name: PathExprSegment<'a>,

    /// The parameters to the method
    pub parameters: CallParams<'a>,
}
