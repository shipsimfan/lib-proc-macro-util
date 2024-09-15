use crate::ast::InnerAttribute;

impl<'a> std::fmt::Display for InnerAttribute<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#![{}]", self.attr)
    }
}
