use crate::{
    ast::{GenericArguments, Punctuated},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

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

impl Generics {
    /// Convert these [`Generics`] to their corrisponding [`GenericArguments`]
    pub fn to_args(&self) -> GenericArguments {
        let mut arguments = Punctuated::new();
        for (generic, comma) in &self.arguments {
            arguments.push_element(generic.to_arg());
            if let Some(comma) = comma {
                arguments.push_seperator(comma.clone());
            }
        }

        GenericArguments {
            leading_double_colon: None,
            left_triangle: self.left_triangle.clone(),
            arguments,
            right_triangle: self.right_triangle.clone(),
        }
    }
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
