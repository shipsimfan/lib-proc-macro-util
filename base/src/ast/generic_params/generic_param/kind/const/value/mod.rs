use crate::{
    ast::expressions::BlockExpression,
    tokens::{Identifier, Literal},
    Token,
};

pub enum ConstParamValue<'a> {
    Block(BlockExpression<'a>),
    Identifier(Identifier),
    Literal(Option<Token![-]>, Literal),
}
