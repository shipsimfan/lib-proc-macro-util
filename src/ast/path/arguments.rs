use crate::ast::{AngleBracketGenerics, ParenthesisGenerics};

#[derive(Clone)]
pub enum PathArguments {
    None,
    AngleBrackets(AngleBracketGenerics),
    Parentheses(ParenthesisGenerics),
}
