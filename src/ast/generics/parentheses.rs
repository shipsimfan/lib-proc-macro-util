use crate::{
    ast::{Punctuated, ReturnType, Type},
    tokens::{Comma, Parenthesis},
};

#[derive(Clone)]
pub struct ParenthesisGenerics {
    pub parentheses: Parenthesis,
    pub inputs: Punctuated<Type, Comma>,
    pub output: ReturnType,
}
