use crate::{ast::expressions::ElseBlockExpression, Generator, ToTokens};

impl<'a> ToTokens for ElseBlockExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ElseBlockExpression::If(r#if) => r#if.to_tokens(generator),
            ElseBlockExpression::Block(block) => block.to_tokens(generator),
        }
    }
}
