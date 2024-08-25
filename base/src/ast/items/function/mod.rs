use crate::{
    ast::{Generics, Meta, Punctuated, Visibility},
    tokens::Identifier,
    Error, Generator, Parser, Result, ToTokens, Token,
};

mod body;
mod parameter;
mod return_type;

pub use body::FunctionBody;
pub use parameter::FunctionParameter;
use proc_macro::Delimiter;
pub use return_type::FunctionReturnType;

/// A declaration of a function
///
/// Example: `fn example() -> u8 { 10 }`
#[derive(Debug, Clone)]
pub struct Function<'a> {
    /// The metadata about the function
    pub meta: Vec<Meta<'a>>,

    /// The visibility of this function
    pub visibility: Option<Visibility<'a>>,

    /// The `fn` token itself
    pub r#fn: Token![fn],

    /// The [`Identifier`] naming this function
    pub identifier: Identifier,

    /// The generics for this function
    pub generics: Option<Generics>,

    /// The parameters passed into this function
    pub parameters: Punctuated<FunctionParameter, Token![,]>,

    /// The type returned by this function
    pub return_type: Option<FunctionReturnType>,

    /// The body of the function
    pub body: FunctionBody<'a>,
}

impl<'a> Function<'a> {
    /// Parses the [`Fn`] following `visibility`
    pub fn parse(
        meta: Vec<Meta<'a>>,
        visibility: Option<Visibility<'a>>,
        parser: &mut Parser<'a>,
    ) -> Result<Self> {
        let r#fn = parser.parse()?;
        let identifier = parser.parse()?;
        let generics = parser.parse()?;

        let parameters_group = match parser.group() {
            Some(group) => match group.delimiter() {
                Delimiter::Parenthesis => group,
                _ => {
                    return Err(Error::new_at(
                        "function parameters must be surrounded by parenthesis",
                        group.span(),
                    ))
                }
            },
            None => return Err(Error::new_at("expected function parameters", parser.span())),
        };

        let parameters = Punctuated::parse(&mut parameters_group.tokens(), false, true)?;
        let return_type = parser.parse()?;
        let body = parser.parse()?;

        Ok(Function {
            meta,
            visibility,
            r#fn,
            identifier,
            generics,
            parameters,
            return_type,
            body,
        })
    }
}

impl<'a> ToTokens for Function<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.meta.to_tokens(generator);
        self.visibility.to_tokens(generator);
        self.r#fn.to_tokens(generator);
        self.identifier.to_tokens(generator);
        self.generics.to_tokens(generator);
        self.parameters
            .to_tokens(&mut generator.group(Delimiter::Parenthesis));
        self.return_type.to_tokens(generator);
        self.body.to_tokens(generator);
    }
}
