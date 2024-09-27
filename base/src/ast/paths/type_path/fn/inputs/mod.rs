use crate::{tokens::Type, Token};

pub struct TypePathFnInputs {
    pub first: Type,
    pub remaining: Vec<(Token![,], Type)>,
    pub end: Option<Token![,]>,
}
