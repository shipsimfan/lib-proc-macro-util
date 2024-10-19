use crate::Token;

mod item;

mod parse;
mod to_tokens;

pub use item::WhereClauseItem;

/// A clause restricting generic arguments associated with an item
#[derive(Debug, Clone)]
pub struct WhereClause<'a> {
    /// The `where` token indicating this clause
    pub r#where: Token![where],

    /// The set of items which are followed by commas
    pub initial: Vec<(WhereClauseItem<'a>, Token![,])>,

    /// A final item not followed by a comma
    pub r#final: Option<WhereClauseItem<'a>>,
}
