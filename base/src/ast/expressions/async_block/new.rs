use crate::{
    ast::expressions::{AsyncBlockExpression, BlockExpression},
    Token,
};

impl<'a> AsyncBlockExpression<'a> {
    /// Creates a new empty [`AsyncBlockExpression`]
    pub fn new() -> Self {
        AsyncBlockExpression {
            r#async: Token![async](),
            r#move: None,
            block: BlockExpression::new(),
        }
    }
}
