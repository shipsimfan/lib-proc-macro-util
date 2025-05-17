use crate::{
    ast::{expressions::BlockExpression, Expression, TypeNoBounds},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// The body of a closure
#[derive(Debug, Clone)]
pub enum ClosureBody<'a> {
    /// The body is a single expression
    Expression(Box<Expression<'a>>),

    /// The body has a defined return type
    ReturnType {
        /// The arrow leading the return type
        arrow: Token![->],

        /// The type being returned
        r#type: Box<TypeNoBounds<'a>>,

        /// The expression making up the body of the closure
        expression: BlockExpression<'a>,
    },
}
