use crate::{
    ast::{Attribute, Pattern},
    tokens::{Colon, Comma, TripleDot},
};

#[derive(Clone)]
pub struct Variadic {
    pub attributes: Vec<Attribute>,
    pub pattern: Option<(Box<Pattern>, Colon)>,
    pub dots: TripleDot,
    pub comma: Option<Comma>,
}
