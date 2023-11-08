use crate::{
    ast::{BoundLifetimes, Path, TraitBoundModifier},
    tokens::Parenthesis,
};

#[derive(Clone)]
pub struct TraitBound {
    pub parentheses: Option<Parenthesis>,
    pub modifier: TraitBoundModifier,
    pub lifetimes: Option<BoundLifetimes>,
    pub path: Path,
}
