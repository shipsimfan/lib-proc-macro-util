use crate::{
    ast::expressions::{StructBase, StructExprField},
    Token,
};

/// The fields which make up a struct expression
#[derive(Debug, Clone)]
pub struct StructExprFields<'a> {
    /// The first field in the struct
    pub first: StructExprField<'a>,

    /// The remaining fields and their separators
    pub remaining: Vec<(Token![,], StructExprField<'a>)>,

    /// A base expression to fill out the remaining fields
    pub base: Option<(Token![,], StructBase<'a>)>,

    /// An optional final comma
    pub last: Option<Token![,]>,
}
