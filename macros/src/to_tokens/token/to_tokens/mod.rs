use super::Token;
use proc_macro_util_base::{tokens::Identifier, Generator};

mod identifier;
mod literal;
mod punctuation;
mod variable;

impl<'a> Token<'a> {
    /// Generates the tokens needed to generate this token
    pub fn to_tokens(self, generator: &mut Generator, generator_name: &Identifier) {
        match self {
            Token::Group(group) => group.to_tokens(generator_name, generator),
            Token::Identifier(identifier) => {
                identifier::to_tokens(identifier, generator_name, generator)
            }
            Token::Literal(literal) => literal::to_tokens(literal, generator_name, generator),
            Token::Punctuation(punctuation) => {
                punctuation::to_tokens(punctuation, generator_name, generator)
            }
            Token::Variable(identifier) => {
                variable::to_tokens(identifier, generator_name, generator)
            }
        }
    }
}
