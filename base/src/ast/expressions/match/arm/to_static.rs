use crate::ast::{
    expressions::{MatchArm, MatchArmGuard},
    OuterAttribute,
};

impl<'a> MatchArm<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MatchArm<'static> {
        MatchArm {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            pattern: self.pattern.into_static(),
            guard: self.guard.map(MatchArmGuard::into_static),
        }
    }
}
