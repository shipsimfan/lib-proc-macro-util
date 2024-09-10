use crate::Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.errors.len() {
            write!(f, "{}", self.errors[i])?;

            if i < self.errors.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.errors.len() {
            write!(f, "{}", self.errors[i])?;

            if i < self.errors.len() - 1 {
                write!(f, ";")?;
            }
        }

        Ok(())
    }
}
