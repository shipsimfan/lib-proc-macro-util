use crate::{ast::Expression, Token};

mod to_static;
mod to_tokens;

/// Waits for a future to complete
#[derive(Debug, Clone)]
pub struct AwaitExpression<'a> {
    /// The expression to wait on completion
    pub expression: Box<Expression<'a>>,

    /// The dot indicating this may be an await
    pub dot: Token![.],

    /// The `await` keyword indicating this is an await expression
    pub r#await: Token![await],
}
