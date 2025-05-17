use crate::ast::expressions::{StructExprStruct, StructExprTuple};

mod parse;
mod to_static;
mod to_tokens;

/// The kind of fields that make up a struct
#[derive(Debug, Clone)]
pub enum StructExprKind<'a> {
    /// The fields are defined like a struct
    Struct(Option<StructExprStruct<'a>>),

    /// The fields are defined like a tuple
    Tuple(Option<StructExprTuple<'a>>),

    /// There are no fields
    Unit,
}
