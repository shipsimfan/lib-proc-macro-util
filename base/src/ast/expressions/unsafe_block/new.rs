use crate::{
    ast::expressions::{BlockExpression, UnsafeBlockExpression},
    Token,
};

impl<'a> UnsafeBlockExpression<'a> {
    /// Creates a new empty [`UnsafeBlockExpression`]
    pub fn new() -> Self {
        UnsafeBlockExpression {
            r#unsafe: Token![unsafe](),
            block: BlockExpression::new(),
        }
    }
}
