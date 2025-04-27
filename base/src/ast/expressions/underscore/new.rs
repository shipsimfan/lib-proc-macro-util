use crate::{ast::expressions::UnderscoreExpression, Token};

impl UnderscoreExpression {
    /// Creates a new empty [`UnderscoreExpression`]
    pub fn new() -> Self {
        UnderscoreExpression {
            underscore: Token![_](),
        }
    }
}
