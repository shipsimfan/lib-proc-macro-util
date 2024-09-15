use crate::ast::OuterAttribute;

impl<'a> std::fmt::Display for OuterAttribute<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#[{}]", self.attr)
    }
}
