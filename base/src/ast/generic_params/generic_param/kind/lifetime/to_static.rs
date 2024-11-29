use crate::ast::LifetimeParam;

impl<'a> LifetimeParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> LifetimeParam<'static> {
        LifetimeParam {
            lifetime: self.lifetime.into_static(),
            bounds: self
                .bounds
                .map(|(colon, bounds)| (colon, bounds.into_static())),
        }
    }
}
