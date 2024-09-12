use crate::tokens::Identifier;

impl From<proc_macro::Ident> for Identifier {
    fn from(ident: proc_macro::Ident) -> Self {
        Identifier::new_raw(ident)
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier::new(value)
    }
}
