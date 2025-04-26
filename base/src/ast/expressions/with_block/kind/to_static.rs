use crate::ast::ExpressionWithBlockKind;

impl<'a> ExpressionWithBlockKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionWithBlockKind<'static> {
        match self {
            ExpressionWithBlockKind::Block(block) => {
                ExpressionWithBlockKind::Block(block.into_static())
            }
            ExpressionWithBlockKind::Unsafe(r#unsafe) => {
                ExpressionWithBlockKind::Unsafe(r#unsafe.into_static())
            }
        }
    }
}
