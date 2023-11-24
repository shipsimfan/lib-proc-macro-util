use proc_macro::Delimiter;

use crate::{
    ast::{FunctionCallExpression, MacroCallExpression},
    tokens::{Group, Identifier, Literal},
    Parse, ToTokens,
};

/// A Rust expression
#[derive(Clone)]
pub enum Expression<'a> {
    /// A block of statements
    Block(Group<'a>),

    /// An invocation of a function
    FunctionCall(FunctionCallExpression<'a>),

    /// An identifier, usually a variable reference
    Identifier(Identifier),

    /// A literal
    Literal(Literal),

    /// An invocation of a macro
    MacroCall(MacroCallExpression<'a>),

    /// A slice
    Slice(Group<'a>),
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut crate::Parser<'a>) -> crate::Result<Self> {
        if parser.peek::<MacroCallExpression>() {
            parser
                .parse()
                .map(|macro_call| Expression::MacroCall(macro_call))
        } else if parser.peek::<FunctionCallExpression>() {
            parser
                .parse()
                .map(|function_call| Expression::FunctionCall(function_call))
        } else if let Some(group) = parser.group() {
            match group.delimiter() {
                Delimiter::Brace => Ok(Expression::Block(group)),
                Delimiter::Bracket => Ok(Expression::Slice(group)),
                _ => Err(parser.error("expected an expression")),
            }
        } else if let Some(identifier) = parser.identifier() {
            Ok(Expression::Identifier(identifier))
        } else if let Some(literal) = parser.literal() {
            Ok(Expression::Literal(literal))
        } else {
            Err(parser.error("expected an expression"))
        }
    }
}

impl<'a> ToTokens for Expression<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        match self {
            Expression::Block(block) => block.to_tokens(generator),
            Expression::FunctionCall(function_call) => function_call.to_tokens(generator),
            Expression::Identifier(identifier) => identifier.to_tokens(generator),
            Expression::Literal(literal) => literal.to_tokens(generator),
            Expression::MacroCall(macro_call) => macro_call.to_tokens(generator),
            Expression::Slice(slice) => slice.to_tokens(generator),
        }
    }
}
