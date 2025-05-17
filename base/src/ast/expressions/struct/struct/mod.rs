use crate::Token;

mod base;
mod field;

mod parse;
mod to_static;
mod to_tokens;

pub use base::StructBase;
pub use field::{StructExprField, StructExprFieldName};

/// A struct expression which is of a struct-style
#[derive(Debug, Clone)]
pub struct StructExprStruct<'a> {
    /// The first field in the struct
    pub first: StructExprField<'a>,

    /// The remaining fields and their separators
    pub remaining: Vec<(Token![,], StructExprField<'a>)>,

    /// A base expression to fill out the remaining fields
    pub base: Option<(Token![,], Option<StructBase<'a>>)>,
}
