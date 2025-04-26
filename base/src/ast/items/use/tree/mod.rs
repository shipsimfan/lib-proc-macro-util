use crate::{
    ast::{MaybeIdentifier, SimplePath},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A subtree of a use declaration
#[derive(Debug, Clone)]
pub enum UseTree<'a> {
    /// Imports all elements from the target
    All {
        /// The path to import from
        prefix: Option<(Option<SimplePath<'a>>, Token![::])>,

        /// The `*` marking this to import all
        asterick: Token![*],
    },

    /// Import certain elements from a subtree of an element
    SubTree {
        /// The path to import from
        prefix: Option<(Option<SimplePath<'a>>, Token![::])>,

        /// Subtrees containing items to import
        trees: Option<(
            Box<UseTree<'a>>,
            Vec<(Token![,], UseTree<'a>)>,
            Option<Token![,]>,
        )>,
    },

    /// An final item to import
    Leaf {
        /// The path to import
        path: SimplePath<'a>,

        /// An optional alias of the type
        r#as: Option<(Token![as], MaybeIdentifier<'a>)>,
    },
}
