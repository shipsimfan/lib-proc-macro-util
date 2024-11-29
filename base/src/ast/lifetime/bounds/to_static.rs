use crate::ast::{Lifetime, LifetimeBounds};

impl<'a> LifetimeBounds<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LifetimeBounds<'static> {
        LifetimeBounds {
            leading: self
                .leading
                .into_iter()
                .map(|(lifetime, plus)| (lifetime.into_static(), plus))
                .collect(),
            ending: self.ending.map(Lifetime::into_static),
        }
    }
}
