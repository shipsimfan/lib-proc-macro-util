use crate::{
    ast::{ForLifetimes, Lifetime, LifetimeBounds, Type, TypeParamBounds},
    Token,
};

#[derive(Debug, Clone)]
pub enum WhereClauseItem<'a> {
    Lifetime {
        lifetime: Lifetime<'a>,
        colon: Token![:],
        bounds: LifetimeBounds<'a>,
    },
    Type {
        for_lifetimes: Option<ForLifetimes<'a>>,
        r#type: Type<'a>,
        colon: Token![:],
        bounds: Option<TypeParamBounds<'a>>,
    },
}
