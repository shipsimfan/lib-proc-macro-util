use crate::{ast::ExpressionWithBlockKind, Generator, ToTokens};

impl<'a> ToTokens for ExpressionWithBlockKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ExpressionWithBlockKind::Block(block) => block.to_tokens(generator),
            ExpressionWithBlockKind::Unsafe(r#unsafe) => r#unsafe.to_tokens(generator),
            ExpressionWithBlockKind::Const(r#const) => r#const.to_tokens(generator),
            ExpressionWithBlockKind::Loop(r#loop) => r#loop.to_tokens(generator),
            ExpressionWithBlockKind::If(r#if) => r#if.to_tokens(generator),
            ExpressionWithBlockKind::Match(r#match) => r#match.to_tokens(generator),
        }
    }
}
