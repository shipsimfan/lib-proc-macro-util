use crate::ast::{Expression, ExpressionKind, OuterAttribute};

impl<'a> Expression<'a> {
    /// Creates a new [`Expression`]
    pub fn new<T: Into<ExpressionKind<'a>>>(attributes: Vec<OuterAttribute<'a>>, kind: T) -> Self {
        Expression {
            attributes,
            kind: kind.into(),
        }
    }
}
