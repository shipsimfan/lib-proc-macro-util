use crate::{
    ast::{Field, Punctuated},
    tokens::{Brace, Comma},
};

#[derive(Clone)]
pub struct FieldsNamed {
    pub brace: Brace,
    pub named: Punctuated<Field, Comma>,
}
