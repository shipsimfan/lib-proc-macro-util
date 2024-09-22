use crate::{
    ast::{DelimTokenTree, MacroInvocation, SimplePath},
    Token,
};

impl<'a> MacroInvocation<'a> {
    /// Create a new [`MacroInvocation`]
    pub fn new<T: Into<DelimTokenTree<'a>>>(path: SimplePath<'a>, group: T) -> Self {
        MacroInvocation {
            path,
            exclamation: Token![!](),
            group: group.into(),
        }
    }
}
