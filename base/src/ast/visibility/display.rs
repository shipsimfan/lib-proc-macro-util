use crate::ast::Visibility;

impl<'a> std::fmt::Display for Visibility<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.r#pub.fmt(f)?;
        if let Some(scope) = &self.scope {
            scope.fmt(f)?;
        }
        Ok(())
    }
}
