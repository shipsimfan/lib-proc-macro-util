use crate::{ast::Path, tokens::Group, Parse, Parser, ToTokens};

/// A function call expression
///
/// Example: `foo("test", 0)`
#[derive(Clone)]
pub struct FunctionCallExpression<'a> {
    /// The path to the function
    pub path: Path,

    /// The parameters passed to the function
    pub parameters: Group<'a>,
}

impl<'a> Parse<'a> for FunctionCallExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> crate::Result<Self> {
        let path = parser.parse()?;
        let parameters = parser.parse()?;

        Ok(FunctionCallExpression { path, parameters })
    }
}

impl<'a> ToTokens for FunctionCallExpression<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.path.to_tokens(generator);
        self.parameters.to_tokens(generator);
    }
}
