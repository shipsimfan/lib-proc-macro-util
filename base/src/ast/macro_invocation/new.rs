use crate::{
    ast::{MacroInvocation, SimplePath},
    tokens::Group,
    Token,
};
use std::borrow::Cow;

impl<'a> MacroInvocation<'a> {
    /// Create a new [`MacroInvocation`]
    pub fn new<T: Into<Cow<'a, Group>>>(path: SimplePath<'a>, group: T) -> Self {
        MacroInvocation {
            path,
            exclamation: Token![!](),
            group: group.into(),
        }
    }
}
