use crate::{
    ast::{DelimTokenTree, MacroInvocationSemi, SimplePath},
    Token,
};
use proc_macro::Delimiter;

impl<'a> MacroInvocationSemi<'a> {
    /// Create a new [`MacroInvocationSemi`]
    pub fn new<T: Into<DelimTokenTree<'a>>>(path: SimplePath<'a>, group: T) -> Self {
        let group = group.into();
        match group.delimiter {
            Delimiter::Parenthesis | Delimiter::Bracket => {
                MacroInvocationSemi::ParenthesesOrBracket(path, Token![!](), group, Token![;]())
            }
            Delimiter::Brace => MacroInvocationSemi::Brace(path, Token![!](), group),
            Delimiter::None => unimplemented!(),
        }
    }
}
