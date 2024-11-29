use crate::{
    ast::{ForLifetimes, Lifetime, LifetimeBounds, Type, TypeParamBounds},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// An item in a where clause
#[derive(Debug, Clone)]
pub enum WhereClauseItem<'a> {
    /// This item is restricting a lifetime
    Lifetime {
        /// The lifetime being restricted
        lifetime: Lifetime<'a>,

        /// Colon separating lifetime and bounds
        colon: Token![:],

        /// Bounds restircting the lifetime
        bounds: LifetimeBounds<'a>,
    },

    /// This item is restricting a type
    Type {
        /// The lifetimes the type must be valid for
        for_lifetimes: Option<ForLifetimes<'a>>,

        /// The type being restricted
        r#type: Type<'a>,

        /// Colon separating type and bounds
        colon: Token![:],

        /// Bounds restricting the type
        bounds: Option<TypeParamBounds<'a>>,
    },
}
