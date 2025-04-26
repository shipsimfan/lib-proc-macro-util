use crate::{
    ast::{InnerAttribute, Item},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// The body of a module
#[derive(Debug, Clone)]
pub enum ModuleBody<'a> {
    /// A semi-colon marking no inline body
    None(Token![;]),

    /// An inline body
    Some {
        /// Attributes modifying this module
        attributes: Vec<InnerAttribute<'a>>,

        /// The items making up this module
        items: Vec<Item<'a>>,
    },
}
