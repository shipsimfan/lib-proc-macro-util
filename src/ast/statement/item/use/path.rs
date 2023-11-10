use crate::{
    ast::UseTree,
    tokens::{DoubleColon, Ident},
};

#[derive(Clone)]
pub struct UsePath {
    pub ident: Ident,
    pub double_colon: DoubleColon,
    pub tree: Box<UseTree>,
}
