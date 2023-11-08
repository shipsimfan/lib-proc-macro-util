use super::Field;
use crate::{
    ast::Punctuated,
    tokens::{Comma, Parenthesis},
};

#[derive(Clone)]
pub struct FieldsUnnamed {
    pub parentheses: Parenthesis,
    pub unnamed: Punctuated<Field, Comma>,
}
