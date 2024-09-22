use crate::{ast::DelimTokenTree, Parse, Parser, Result};

impl<'a> Parse<'a> for DelimTokenTree<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse().map(|group| DelimTokenTree::Borrowed(group))
    }
}
