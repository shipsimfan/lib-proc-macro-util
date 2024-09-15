use crate::ast::Attr;

impl<'a> std::fmt::Display for Attr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path.fmt(f)?;
        if let Some(input) = &self.input {
            input.fmt(f)?;
        }
        Ok(())
    }
}
