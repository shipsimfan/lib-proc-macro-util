use crate::{ast::Attr, Token};

mod from;
mod new;
mod parse;
mod to_tokens;

/// Outer attributes are [`Attr`]s which apply to the item follows them
#[derive(Debug, Clone)]
pub struct OuterAttribute<'a> {
    /// The "#" identifying the attribute
    pub hash: Token![#],

    /// The content of the attribute
    pub attr: Attr<'a>,
}
