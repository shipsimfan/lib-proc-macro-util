use crate::Token;

mod field;

pub use field::TupleField;

#[derive(Debug, Clone)]
pub struct TupleFields<'a> {
    pub first: TupleField<'a>,
    pub remaining: Vec<(Token![,], TupleField<'a>)>,
    pub last: Option<Token![,]>,
}
