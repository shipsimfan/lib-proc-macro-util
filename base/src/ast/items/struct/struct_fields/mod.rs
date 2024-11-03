use crate::Token;

mod field;

pub use field::StructField;

#[derive(Debug, Clone)]
pub struct StructFields<'a> {
    pub first: StructField<'a>,
    pub remaining: Vec<(Token![,], StructField<'a>)>,
    pub last: Option<Token![,]>,
}
