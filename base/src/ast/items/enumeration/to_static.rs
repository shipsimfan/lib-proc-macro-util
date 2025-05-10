use crate::ast::{
    items::{EnumItems, Enumeration},
    GenericParams, WhereClause,
};
use std::borrow::Cow;

impl<'a> Enumeration<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Enumeration<'static> {
        Enumeration {
            r#enum: self.r#enum,
            name: Cow::Owned(match self.name {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            }),
            generic_params: self.generic_params.map(GenericParams::into_static),
            where_clause: self.where_clause.map(WhereClause::into_static),
            enum_items: self.enum_items.map(EnumItems::into_static),
        }
    }
}
