use crate::{
    ast::{expressions::BlockExpression, ConstParamValue},
    tokens::{Identifier, Literal},
    Token,
};

impl<'a> From<BlockExpression<'a>> for ConstParamValue<'a> {
    fn from(block: BlockExpression<'a>) -> Self {
        ConstParamValue::Block(block)
    }
}

impl<'a> From<Identifier> for ConstParamValue<'a> {
    fn from(identifier: Identifier) -> Self {
        ConstParamValue::IdentifierOwned(identifier)
    }
}

impl<'a> From<&'a Identifier> for ConstParamValue<'a> {
    fn from(identifier: &'a Identifier) -> Self {
        ConstParamValue::Identifier(identifier)
    }
}

impl<'a> From<Literal> for ConstParamValue<'a> {
    fn from(literal: Literal) -> Self {
        ConstParamValue::LiteralOwned(None, literal)
    }
}

impl<'a> From<(Token![-], Literal)> for ConstParamValue<'a> {
    fn from(value: (Token![-], Literal)) -> Self {
        ConstParamValue::LiteralOwned(Some(value.0), value.1)
    }
}

impl<'a> From<&'a Literal> for ConstParamValue<'a> {
    fn from(literal: &'a Literal) -> Self {
        ConstParamValue::Literal(None, literal)
    }
}

impl<'a> From<(Token![-], &'a Literal)> for ConstParamValue<'a> {
    fn from(value: (Token![-], &'a Literal)) -> Self {
        ConstParamValue::Literal(Some(value.0), value.1)
    }
}
