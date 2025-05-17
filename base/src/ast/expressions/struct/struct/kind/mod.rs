use crate::ast::expressions::{StructExprFields, StructExprTuple};

/// The kind of fields that make up a struct
#[derive(Debug, Clone)]
pub enum StructExprKind<'a> {
    /// The fields are defined like a struct
    Struct(StructExprFields<'a>),

    /// The fields are defined like a tuple
    Tuple(StructExprTuple<'a>),

    /// There are no fields
    Unit,
}
