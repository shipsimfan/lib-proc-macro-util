use crate::{ast::Path, Parse, Parser, Result, ToTokens};

/// A Rust type
pub struct Type {
    path: Path,
}

impl<'a> Parse<'a> for Type {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;

        Ok(Type { path })
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.path.to_tokens(generator);
    }
}
