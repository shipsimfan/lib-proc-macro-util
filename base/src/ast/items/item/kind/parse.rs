use crate::{ast::ItemKind, Parse, Parser, Result};

impl<'a> Parse<'a> for ItemKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(macro_item) = parser.step_parse() {
            return Ok(ItemKind::Macro(macro_item));
        }

        parser.parse().map(|vis_item| ItemKind::Vis(vis_item))
    }
}
