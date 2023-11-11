use proc_macro::{Spacing, Span};

use crate::ToTokens;

/// A single character of punctuation
#[derive(Clone)]
pub struct Punctuation(proc_macro::Punct);

impl Punctuation {
    /// Creates a new [`Punctuation`] token
    ///
    /// ## Parameters
    ///  * `c` - The [`char`] for the new punctuation
    ///  * `spacing` - The [`Spacing`] for the new punctuation
    ///  * `span` - The [`Span`] for the new punctuation
    ///
    /// ## Return Value
    /// Returns the newly created [`Punctuation`]
    pub fn new(c: char, spacing: Spacing, span: Span) -> Self {
        let mut punct = proc_macro::Punct::new(c, spacing);
        punct.set_span(span);

        Punctuation(punct)
    }
}

impl From<proc_macro::Punct> for Punctuation {
    fn from(punctuation: proc_macro::Punct) -> Self {
        Punctuation(punctuation)
    }
}

impl Into<proc_macro::Punct> for Punctuation {
    fn into(self) -> proc_macro::Punct {
        self.0
    }
}

impl ToTokens for Punctuation {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        generator.punctuation(self.clone())
    }
}
