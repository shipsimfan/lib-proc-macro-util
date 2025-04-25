use crate::ast::expressions::CallParams;

impl<'a> CallParams<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> CallParams<'static> {
        CallParams {
            first: self.first.map(|first| Box::new(first.into_static())),
            remaining: self
                .remaining
                .into_iter()
                .map(|(comma, expression)| (comma, expression.into_static()))
                .collect(),
            last: self.last,
        }
    }
}
