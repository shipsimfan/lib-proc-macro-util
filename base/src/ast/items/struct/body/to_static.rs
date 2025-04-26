use crate::ast::{
    items::{StructBody, StructFields, TupleFields},
    WhereClause,
};

impl<'a> StructBody<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructBody<'static> {
        match self {
            StructBody::Normal {
                where_clause,
                fields,
            } => StructBody::Normal {
                where_clause: where_clause.map(WhereClause::into_static),
                fields: fields.map(StructFields::into_static),
            },
            StructBody::Tuple {
                fields,
                where_clause,
                semi,
            } => StructBody::Tuple {
                where_clause: where_clause.map(WhereClause::into_static),
                fields: fields.map(TupleFields::into_static),
                semi,
            },
            StructBody::Empty { where_clause, semi } => StructBody::Empty {
                where_clause: where_clause.map(WhereClause::into_static),
                semi,
            },
        }
    }
}
