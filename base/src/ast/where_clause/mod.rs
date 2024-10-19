use crate::Token;

mod item;

pub use item::WhereClauseItem;

#[derive(Debug, Clone)]
pub struct WhereClause<'a> {
    pub r#where: Token![where],
    pub initial: Vec<(WhereClauseItem<'a>, Token![,])>,
    pub r#final: Option<WhereClauseItem<'a>>,
}
