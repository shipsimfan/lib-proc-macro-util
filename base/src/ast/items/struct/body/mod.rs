use crate::{
    ast::{
        items::{StructFields, TupleFields},
        WhereClause,
    },
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A body of a structure
#[derive(Debug, Clone)]
pub enum StructBody<'a> {
    /// The structure is a normal structure with named fields
    Normal {
        /// A where clause restricting associated generic parameters
        where_clause: Option<WhereClause<'a>>,

        /// The set of named fields
        fields: Option<StructFields<'a>>,
    },

    /// The tuple is structure type with unnamed fields
    Tuple {
        /// The set of unnamed fields
        fields: Option<TupleFields<'a>>,

        /// A where clause restricting associated generic parameters
        where_clause: Option<WhereClause<'a>>,

        /// The final semi colon
        semi: Token![;],
    },

    /// The structure contains no fields
    Empty {
        /// A where clause restricting associated generic parameters
        where_clause: Option<WhereClause<'a>>,

        /// The final semi colon
        semi: Token![;],
    },
}
