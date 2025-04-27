use crate::{
    ast::expressions::{BlockExpression, ConstBlockExpression},
    Token,
};

impl<'a> ConstBlockExpression<'a> {
    /// Creates a new empty [`ConstBlockExpression`]
    pub fn new() -> Self {
        ConstBlockExpression {
            r#const: Token![const](),
            block: BlockExpression::new(),
        }
    }
}
