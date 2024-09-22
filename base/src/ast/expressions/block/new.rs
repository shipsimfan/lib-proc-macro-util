use crate::ast::expressions::BlockExpression;

impl<'a> BlockExpression<'a> {
    /// Creates a new empty [`BlockExpression`]
    pub const fn new() -> Self {
        BlockExpression {
            attributes: Vec::new(),
            statements: Vec::new(),
            end: None,
        }
    }
}
