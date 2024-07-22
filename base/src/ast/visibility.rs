use crate::{tokens::Group, Generator, Parse, Parser, Result, ToTokens, Token};

/// The visibility of an item
///
/// Example: `pub(crate)`
#[derive(Debug, Clone)]
pub struct Visibility<'a> {
    /// The public keyword identifying this construct
    pub r#pub: Token![pub],

    /// The specifier restricting the scope of the public
    pub specifier: Option<Group<'a>>,
}

impl<'a> Parse<'a> for Visibility<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let r#pub = parser.parse()?;
        let specifier = parser.parse()?;

        Ok(Visibility { r#pub, specifier })
    }
}

impl<'a> ToTokens for Visibility<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.r#pub.to_tokens(generator);
        self.specifier.to_tokens(generator);
    }
}
