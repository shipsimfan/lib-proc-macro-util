use crate::ast::{ExpressionWithoutBlock, OuterAttribute};

impl<'a> ExpressionWithoutBlock<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionWithoutBlock<'static> {
        ExpressionWithoutBlock {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            kind: self.kind.into_static(),
        }
    }
}
