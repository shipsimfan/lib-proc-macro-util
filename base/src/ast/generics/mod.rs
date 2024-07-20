use crate::{ast::Punctuated, Generator, Parse, Parser, Result, ToTokens, Token};

mod generic;
mod lifetime;
mod r#type;

pub use generic::Generic;
pub use lifetime::*;
pub use r#type::*;

/// A list of [`Generic`]s
#[derive(Clone)]
pub struct Generics {
    /// The start of the generic arguments
    pub left_triangle: Token![<],

    /// The generic arguments themselves
    pub arguments: Punctuated<Generic, Token![,]>,

    /// The end of the generic arguments
    pub right_triangle: Token![>],
}

impl<'a> Parse<'a> for Generics {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let left_triangle = parser.parse()?;
        let arguments = Punctuated::parse(parser, true)?;
        let right_triangle = parser.parse()?;

        Ok(Generics {
            left_triangle,
            arguments,
            right_triangle,
        })
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, generator: &mut Generator) {
        self.left_triangle.to_tokens(generator);
        self.arguments.to_tokens(generator);
        self.right_triangle.to_tokens(generator);
    }
}
