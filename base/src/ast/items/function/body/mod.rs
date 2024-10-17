use crate::{ast::expressions::BlockExpression, Token};

#[derive(Debug, Clone)]
pub enum FunctionBody<'a> {
    Expression(BlockExpression<'a>),
    Semi(Token![;]),
}
