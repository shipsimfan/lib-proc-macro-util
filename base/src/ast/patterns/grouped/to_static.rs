use crate::ast::patterns::GroupedPattern;

impl<'a> GroupedPattern<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GroupedPattern<'static> {
        GroupedPattern {
            pattern: Box::new(self.pattern.into_static()),
        }
    }
}
