use crate::ast::PathInExpression;

mod kind;
mod r#struct;
mod tuple;

mod parse;
mod to_static;
mod to_tokens;

pub use kind::StructExprKind;
pub use r#struct::*;
pub use tuple::StructExprTuple;

/// An expression which creates a struct
#[derive(Debug, Clone)]
pub struct StructExpression<'a> {
    /// The path to the struct
    pub path: PathInExpression<'a>,

    /// The kind of struct it is
    pub kind: StructExprKind<'a>,
}
