use crate::{ast::Path, tokens::Group, Parse, Parser, ToTokens, Token};

/// A macro call expression
///
/// Example: `format!("test {}", 0)`
#[derive(Clone)]
pub struct MacroCallExpression<'a> {
    /// The path to the macro
    pub path: Path,

    /// The excalmation mark
    pub exclamation: Token![!],

    /// The body of the macro
    pub group: Group<'a>,
}

impl<'a> Parse<'a> for MacroCallExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> crate::Result<Self> {
        let path = parser.parse()?;
        let exclamation = parser.parse()?;
        let group = parser.parse()?;

        Ok(MacroCallExpression {
            path,
            exclamation,
            group,
        })
    }
}

impl<'a> ToTokens for MacroCallExpression<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.path.to_tokens(generator);
        self.exclamation.to_tokens(generator);
        self.group.to_tokens(generator);
    }
}
