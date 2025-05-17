use crate::ast::PathInExpression;

mod base;
mod field;
mod fields;
mod kind;

pub use base::StructBase;
pub use field::{StructExprField, StructExprFieldName};
pub use fields::StructExprFields;
pub use kind::StructExprKind;

/// An expression which creates a struct
#[derive(Debug, Clone)]
pub struct StructExpression<'a> {
    /// The path to the struct
    pub path: PathInExpression<'a>,

    /// The kind of struct it is
    pub kind: StructExprKind<'a>,
}
