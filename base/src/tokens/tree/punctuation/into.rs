use crate::tokens::Punctuation;

impl Into<proc_macro::Punct> for Punctuation {
    fn into(self) -> proc_macro::Punct {
        self.0
    }
}
