use crate::ast::SimplePath;

impl<'a> std::fmt::Display for SimplePath<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(leading) = &self.leading {
            leading.fmt(f)?;
        }

        self.first.fmt(f)?;

        for (separator, segment) in &self.remaining {
            separator.fmt(f)?;
            segment.fmt(f)?;
        }

        Ok(())
    }
}
