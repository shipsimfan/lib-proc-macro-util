use crate::ast::{ExpressionWithBlock, OuterAttribute};

impl<'a> ExpressionWithBlock<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionWithBlock<'static> {
        ExpressionWithBlock {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            kind: self.kind.into_static(),
        }
    }
}
