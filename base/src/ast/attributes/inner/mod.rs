use crate::{ast::Attr, Token};

mod from;
mod new;
mod parse;
mod to_tokens;

/// Inner attributes are [`Attr`]s which apply to the item they are inside
#[derive(Debug, Clone)]
pub struct InnerAttribute<'a> {
    /// The "#" identifying the attribute
    pub hash: Token![#],

    /// The "!" identifying the inner attribute
    pub exclamation: Token![!],

    /// The content of the attribute
    pub attr: Attr<'a>,
}
