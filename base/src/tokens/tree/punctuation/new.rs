use crate::tokens::Punctuation;

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
