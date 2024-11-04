use crate::Token;

mod field;

mod parse;
mod to_tokens;

pub use field::StructField;

/// A set of fields of a normal structure
#[derive(Debug, Clone)]
pub struct StructFields<'a> {
    /// The first required field
    pub first: StructField<'a>,

    /// The remaining fields of the struct with their separators
    pub remaining: Vec<(Token![,], StructField<'a>)>,

    /// The last optional comma
    pub last: Option<Token![,]>,
}
