use crate::{ast::items::UseTree, Parse, Parser, Result};

impl<'a> Parse<'a> for UseTree<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok((prefix, asterick)) = parser.step_parse() {
            return Ok(UseTree::All { prefix, asterick });
        }

        if let Ok((prefix, trees)) = parser.step_parse() {
            return Ok(UseTree::SubTree { prefix, trees });
        }

        parser
            .parse()
            .map(|(path, r#as)| UseTree::Leaf { path, r#as })
    }
}
