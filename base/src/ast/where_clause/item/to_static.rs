use crate::ast::{ForLifetimes, TypeParamBounds, WhereClauseItem};

impl<'a> WhereClauseItem<'a> {
    pub fn into_static(self) -> WhereClauseItem<'static> {
        match self {
            WhereClauseItem::Lifetime {
                lifetime,
                colon,
                bounds,
            } => WhereClauseItem::Lifetime {
                lifetime: lifetime.into_static(),
                colon,
                bounds: bounds.into_static(),
            },
            WhereClauseItem::Type {
                for_lifetimes,
                r#type,
                colon,
                bounds,
            } => WhereClauseItem::Type {
                for_lifetimes: for_lifetimes.map(ForLifetimes::into_static),
                r#type: r#type.into_static(),
                colon,
                bounds: bounds.map(TypeParamBounds::into_static),
            },
        }
    }
}
