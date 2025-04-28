use crate::ast::expressions::ElseBlockExpression;

impl<'a> ElseBlockExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ElseBlockExpression<'static> {
        match self {
            ElseBlockExpression::If(r#if) => ElseBlockExpression::If(Box::new(r#if.into_static())),
            ElseBlockExpression::Block(block) => ElseBlockExpression::Block(block.into_static()),
        }
    }
}
