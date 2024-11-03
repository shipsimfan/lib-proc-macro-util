use crate::{
    ast::{
        items::{StructFields, TupleFields},
        WhereClause,
    },
    Token,
};

#[derive(Debug, Clone)]
pub enum StructBody<'a> {
    Normal {
        where_clause: Option<WhereClause<'a>>,
        fields: Option<StructFields<'a>>,
    },
    Tuple {
        fields: Option<TupleFields<'a>>,
        where_clause: Option<WhereClause<'a>>,
        semi: Token![;],
    },
    Empty {
        where_clause: Option<WhereClause<'a>>,
        semi: Token![;],
    },
}
