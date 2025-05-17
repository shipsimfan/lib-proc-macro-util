use crate::ast::expressions::ClosureParameters;

impl<'a> ClosureParameters<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ClosureParameters<'static> {
        ClosureParameters {
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(colon, parameter)| (colon, parameter.into_static()))
                .collect(),
            last: self.last,
        }
    }
}
