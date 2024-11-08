use crate::{
    ast::{MacroInvocationSemi, SimplePath},
    tokens::Group,
    Token,
};
use proc_macro::Delimiter;
use std::borrow::Cow;

impl<'a> MacroInvocationSemi<'a> {
    /// Create a new [`MacroInvocationSemi`]
    pub fn new<T: Into<Cow<'a, Group>>>(path: SimplePath<'a>, group: T) -> Self {
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
