use crate::{
    ast::Attribute,
    tokens::{Colon, Comma, Ident, TripleDot},
};

#[derive(Clone)]
pub struct BareVariadic {
    pub attributes: Vec<Attribute>,
    pub name: Option<(Ident, Colon)>,
    pub dots: TripleDot,
    pub comma: Option<Comma>,
}
