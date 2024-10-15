use crate::Token;

mod tree;

mod parse;
mod to_tokens;

pub use tree::UseTree;

/// A statement which imports items from another crate or module
#[derive(Debug, Clone)]
pub struct UseDeclaration<'a> {
    /// The `use` keyword marking this declaration
    pub r#use: Token![use],

    /// The tree of items to import
    pub tree: UseTree<'a>,

    /// The semi-colon marking the end of the item
    pub semi: Token![;],
}
