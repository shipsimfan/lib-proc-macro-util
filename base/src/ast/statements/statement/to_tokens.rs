use crate::{ast::Statement, Generator, ToTokens};

impl<'a> ToTokens for Statement<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            /*Statement::Item(item) => item.to_tokens(generator),
            Statement::Let(r#let) => r#let.to_tokens(generator),*/
            Statement::Expression(expression) => expression.to_tokens(generator),
            Statement::MacroInvocation(macro_invocation) => macro_invocation.to_tokens(generator),
        }
    }
}
