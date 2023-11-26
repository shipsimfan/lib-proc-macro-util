use crate::{
    ast::{Punctuated, Type},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

/// A series of generic arguments
///
/// Example: `<Foo, Bar>`
#[derive(Clone)]
pub struct GenericArguments {
    /// The double colon before the generic arguments
    pub leading_double_colon: Option<Token![::]>,

    /// The start of the generic arguments
    pub left_triangle: Token![<],

    /// The generic arguments themselves
    pub arguments: Punctuated<Type, Token![,]>,

    /// The end of the generic arguments
    pub right_triangle: Token![>],
}

impl<'a> Parse<'a> for GenericArguments {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let leading_double_colon = parser.parse()?;
        let left_triangle = parser.parse()?;

        let mut arguments = Punctuated::new();
        while !parser.peek::<Token![>]>() {
            arguments.push_element(parser.parse()?);

            if parser.peek::<Token![>]>() {
                break;
            }

            arguments.push_seperator(parser.parse()?);
        }

        let right_triangle = parser.parse()?;

        Ok(GenericArguments {
            leading_double_colon,
            left_triangle,
            arguments,
            right_triangle,
        })
    }
}

impl ToTokens for GenericArguments {
    fn to_tokens(&self, generator: &mut Generator) {
        self.leading_double_colon.to_tokens(generator);
        self.left_triangle.to_tokens(generator);
        self.arguments.to_tokens(generator);
        self.right_triangle.to_tokens(generator);
    }
}
