use crate::{ast::Statement, tokens::Brace};

#[derive(Clone)]
pub struct Block {
    pub brace: Brace,
    pub statements: Vec<Statement>,
}
