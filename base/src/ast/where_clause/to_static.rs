use crate::ast::{WhereClause, WhereClauseItem};

impl<'a> WhereClause<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> WhereClause<'static> {
        WhereClause {
            r#where: self.r#where,
            initial: self
                .initial
                .into_iter()
                .map(|(item, separator)| (item.into_static(), separator))
                .collect(),
            r#final: self.r#final.map(WhereClauseItem::into_static),
        }
    }
}
