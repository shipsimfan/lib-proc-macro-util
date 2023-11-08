use crate::{
    ast::{Punctuated, Variant},
    tokens::{Brace, Comma, Enum},
};

#[derive(Clone)]
pub struct DataEnum {
    pub r#enum: Enum,
    pub brace: Brace,
    pub variants: Punctuated<Variant, Comma>,
}
