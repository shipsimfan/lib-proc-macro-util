use crate::ast::{ForLifetimes, TraitBound};

impl<'a> TraitBound<'a> {
    pub fn into_static(self) -> TraitBound<'static> {
        TraitBound {
            delimited: self.delimited,
            question: self.question,
            for_lifetimes: self.for_lifetimes.map(ForLifetimes::into_static),
            path: self.path.into_static(),
        }
    }
}
