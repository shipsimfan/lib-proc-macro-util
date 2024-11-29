use crate::ast::{PathInExpression, QualifiedPathInExpression};

mod parse;
mod to_static;
mod to_tokens;

/// A path to a type of variable in an expression
#[derive(Debug, Clone)]
pub enum PathExpression<'a> {
    /// The path is unqualified
    Normal(PathInExpression<'a>),

    /// The path is qualified
    QualifiedPathInExpression(QualifiedPathInExpression<'a>),
}
