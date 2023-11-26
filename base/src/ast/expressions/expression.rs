use crate::{
    ast::{
        AccessorExpression, FunctionAccessorExpression, FunctionCallExpression,
        MacroCallExpression, ReferenceExpression,
    },
    tokens::{Group, Identifier, Literal},
    Parse, ToTokens, Token,
};
use proc_macro::Delimiter;

/// A Rust expression
#[derive(Clone)]
pub enum Expression<'a> {
    /// An accessor of an item member
    Accessor(AccessorExpression<'a>),

    /// A block of statements
    Block(Group<'a>),

    /// An invocation of a member function
    FunctionAccessor(FunctionAccessorExpression<'a>),

    /// An invocation of a function
    FunctionCall(FunctionCallExpression<'a>),

    /// An identifier, usually a variable reference
    Identifier(Identifier),

    /// A literal
    Literal(Literal),

    /// An invocation of a macro
    MacroCall(MacroCallExpression<'a>),

    /// A reference of another expression
    Reference(ReferenceExpression<'a>),

    /// A slice
    Slice(Group<'a>),
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut crate::Parser<'a>) -> crate::Result<Self> {
        let expression = if parser.peek::<MacroCallExpression>() {
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
        } else if parser.peek::<Token![&]>() {
            parser
                .parse()
                .map(|reference| Expression::Reference(reference))
        } else {
            Err(parser.error("expected an expression"))
        }?;

        if parser.peek::<Token![.]>() {
            if let Ok(function_accessor) = parser.step(|parser| {
                FunctionAccessorExpression::parse(Box::new(expression.clone()), parser)
            }) {
                Ok(Expression::FunctionAccessor(function_accessor))
            } else {
                AccessorExpression::parse(Box::new(expression), parser)
                    .map(|accessor| Expression::Accessor(accessor))
            }
        } else {
            Ok(expression)
        }
    }
}

impl<'a> ToTokens for Expression<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        match self {
            Expression::Accessor(accessor) => accessor.to_tokens(generator),
            Expression::Block(block) => block.to_tokens(generator),
            Expression::FunctionAccessor(function_accessor) => {
                function_accessor.to_tokens(generator)
            }
            Expression::FunctionCall(function_call) => function_call.to_tokens(generator),
            Expression::Identifier(identifier) => identifier.to_tokens(generator),
            Expression::Literal(literal) => literal.to_tokens(generator),
            Expression::MacroCall(macro_call) => macro_call.to_tokens(generator),
            Expression::Reference(reference) => reference.to_tokens(generator),
            Expression::Slice(slice) => slice.to_tokens(generator),
        }
    }
}
