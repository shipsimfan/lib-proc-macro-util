use crate::ast::{ForLifetimes, TraitBound};

impl<'a> TraitBound<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TraitBound<'static> {
        TraitBound {
            delimited: self.delimited,
            question: self.question,
            for_lifetimes: self.for_lifetimes.map(ForLifetimes::into_static),
            path: self.path.into_static(),
        }
    }
}
