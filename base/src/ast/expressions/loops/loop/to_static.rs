use crate::ast::expressions::{LoopExpression, LoopLabel};

impl<'a> LoopExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LoopExpression<'static> {
        LoopExpression {
            label: self.label.map(LoopLabel::into_static),
            kind: self.kind.into_static(),
        }
    }
}
