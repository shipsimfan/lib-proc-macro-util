use crate::tokens::Punctuation;

impl PartialEq<char> for Punctuation {
    fn eq(&self, other: &char) -> bool {
        self.as_char() == *other
    }
}

impl PartialEq for Punctuation {
    fn eq(&self, other: &Self) -> bool {
        self.as_char() == other.as_char() && self.spacing() == other.spacing()
    }
}

impl Eq for Punctuation {}
