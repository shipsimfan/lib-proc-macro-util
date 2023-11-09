use crate::tokens::{Parenthesis, Type};

#[derive(Clone)]
pub struct TypeParentheses {
    pub parentheses: Parenthesis,
    pub element: Box<Type>,
}
