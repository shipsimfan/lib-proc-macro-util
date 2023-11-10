use crate::{
    ast::{Punctuated, UseTree},
    tokens::{Brace, Comma},
};

#[derive(Clone)]
pub struct UseGroup {
    pub brace: Brace,
    pub items: Punctuated<UseTree, Comma>,
}
