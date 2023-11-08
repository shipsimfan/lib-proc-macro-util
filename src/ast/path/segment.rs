use crate::{ast::PathArguments, tokens::Ident};

#[derive(Clone)]
pub struct PathSegment {
    pub ident: Ident,
    pub arguments: PathArguments,
}
