use crate::Token;

mod field;

mod parse;
mod to_static;
mod to_tokens;

pub use field::TupleField;

/// The fields defining a tuple-style struct
#[derive(Debug, Clone)]
pub struct TupleFields<'a> {
    /// The first field
    pub first: TupleField<'a>,

    /// The remaining fields
    pub remaining: Vec<(Token![,], TupleField<'a>)>,

    /// A final optional comma
    pub last: Option<Token![,]>,
}
