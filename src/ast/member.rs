use crate::{ast::Index, tokens::Ident};

#[derive(Clone)]
pub enum Member {
    Named(Ident),
    Unnamed(Index),
}
