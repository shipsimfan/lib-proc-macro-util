use crate::{
    ast::{Lifetime, LifetimeBounds},
    Token,
};

pub struct LifetimeParam<'a> {
    pub lifetime: Lifetime<'a>,
    pub bounds: Option<(Token![:], LifetimeBounds<'a>)>,
}
