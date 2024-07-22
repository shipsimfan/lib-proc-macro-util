use crate::{Error, Generator, Parse, Parser, Result, ToTokens};
use proc_macro::{Spacing, Span};

/// A single character of punctuation
#[derive(Debug, Clone)]
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

    /// Get the [`Span`] of this punctuation
    ///
    /// ## Return Value
    /// Returns this punctuation's [`Span`]
    pub fn span(&self) -> Span {
        self.0.span()
    }

    /// Gets this punctuation as it's [`char`] representation
    ///
    /// ## Return Value
    /// Returns this punctuation as a [`char`]
    pub fn as_char(&self) -> char {
        self.0.as_char()
    }

    /// Get the [`Spacing`] of this punctuation
    ///
    /// ## Return Value
    /// Returns this punctuation's [`Spacing`]
    pub fn spacing(&self) -> Spacing {
        self.0.spacing()
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

impl<'a> Parse<'a> for Punctuation {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .punctuation()
            .ok_or(Error::new("expected punctuation"))
    }
}

impl ToTokens for Punctuation {
    fn to_tokens(&self, generator: &mut Generator) {
        generator.punctuation(self.clone())
    }
}
