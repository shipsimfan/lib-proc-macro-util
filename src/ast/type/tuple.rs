use crate::{
    ast::Punctuated,
    tokens::{Comma, Parenthesis, Type},
};

#[derive(Clone)]
pub struct TypeTuple {
    pub parentheses: Parenthesis,
    pub elements: Punctuated<Type, Comma>,
}
