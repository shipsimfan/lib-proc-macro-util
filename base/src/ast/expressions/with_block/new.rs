use crate::ast::{ExpressionWithBlock, ExpressionWithBlockKind, OuterAttribute};

impl<'a> ExpressionWithBlock<'a> {
    /// Creates a new [`ExpressionWithBlock`]
    pub fn new<T: Into<ExpressionWithBlockKind<'a>>>(
        attributes: Vec<OuterAttribute<'a>>,
        kind: T,
    ) -> Self {
        ExpressionWithBlock {
            attributes,
            kind: kind.into(),
        }
    }
}
