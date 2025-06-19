use crate::ast::ExpressionWithBlockKind;

impl<'a> ExpressionWithBlockKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionWithBlockKind<'static> {
        todo!()
        /*match self {
            ExpressionWithBlockKind::Block(block) => {
                ExpressionWithBlockKind::Block(block.into_static())
            }
            ExpressionWithBlockKind::Unsafe(r#unsafe) => {
                ExpressionWithBlockKind::Unsafe(r#unsafe.into_static())
            }
            ExpressionWithBlockKind::Const(r#const) => {
                ExpressionWithBlockKind::Const(r#const.into_static())
            }
            ExpressionWithBlockKind::Loop(r#loop) => {
                ExpressionWithBlockKind::Loop(r#loop.into_static())
            }
            ExpressionWithBlockKind::If(r#if) => ExpressionWithBlockKind::If(r#if.into_static()),
            ExpressionWithBlockKind::Match(r#match) => {
                ExpressionWithBlockKind::Match(r#match.into_static())
            }
        }*/
    }
}
