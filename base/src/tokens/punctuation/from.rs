use crate::tokens::Punctuation;

impl From<proc_macro::Punct> for Punctuation {
    fn from(punctuation: proc_macro::Punct) -> Self {
        Punctuation::new_raw(punctuation)
    }
}

impl From<char> for Punctuation {
    fn from(c: char) -> Self {
        Punctuation::new(c)
    }
}
