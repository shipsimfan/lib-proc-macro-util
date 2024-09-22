use crate::ast::{ExpressionWithoutBlock, ExpressionWithoutBlockKind, OuterAttribute};

impl<'a> ExpressionWithoutBlock<'a> {
    /// Creates a new [`ExpressionWithoutBlock`]
    pub fn new<T: Into<ExpressionWithoutBlockKind<'a>>>(
        attributes: Vec<OuterAttribute<'a>>,
        kind: T,
    ) -> Self {
        ExpressionWithoutBlock {
            attributes,
            kind: kind.into(),
        }
    }
}
