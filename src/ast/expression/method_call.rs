use crate::{
    ast::{AngleBracketGenerics, Attribute, Expression, Punctuated},
    tokens::{Comma, Dot, Ident, Parenthesis},
};

#[derive(Clone)]
pub struct ExpressionMethodCall {
    pub attributes: Vec<Attribute>,
    pub receiver: Box<Expression>,
    pub dot: Dot,
    pub method: Ident,
    pub generics: Option<AngleBracketGenerics>,
    pub parentheses: Parenthesis,
    pub args: Punctuated<Expression, Comma>,
}
