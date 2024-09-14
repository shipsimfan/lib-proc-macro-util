use crate::tokens::Identifier;

impl<T: AsRef<str> + ?Sized> PartialEq<T> for Identifier {
    fn eq(&self, other: &T) -> bool {
        self.to_string() == other.as_ref()
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Identifier) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Identifier {}
