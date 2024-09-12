use crate::tokens::Punctuation;

impl From<proc_macro::Punct> for Punctuation {
    fn from(punctuation: proc_macro::Punct) -> Self {
        Punctuation(punctuation)
    }
}
