use crate::{tokens::Punctuation, Spacing, Span};

impl Punctuation {
    /// Creates a [`Punctuation`] from `punct`
    pub const fn new_raw(punct: proc_macro::Punct) -> Self {
        Punctuation(punct)
    }

    /// Creates a new [`Punctuation`] token
    ///
    /// ## Parameters
    ///  * `c` - The [`char`] for the new punctuation
    ///  * `spacing` - The [`Spacing`] for the new punctuation
    ///  * `span` - The [`Span`] for the new punctuation
    ///
    /// ## Return Value
    /// Returns the newly created [`Punctuation`]
    pub fn new_spaced_at(c: char, spacing: Spacing, span: Span) -> Self {
        let mut punct = proc_macro::Punct::new(c, spacing);
        punct.set_span(span);

        Punctuation::new_raw(punct)
    }

    /// Creates a new [`Punctuation`] with [`Spacing::Alone`]
    pub fn new_at(c: char, span: Span) -> Self {
        Punctuation::new_spaced_at(c, Spacing::Alone, span)
    }

    /// Creates a new [`Punctuation`] with [`Span::call_site`]
    pub fn new_spaced(c: char, spacing: Spacing) -> Self {
        Punctuation::new_spaced_at(c, spacing, Span::call_site())
    }

    /// Creates a new [`Punctuation`] with [`Spacing::Alone`] and [`Span::call_site`]
    pub fn new(c: char) -> Self {
        Punctuation::new_spaced(c, Spacing::Alone)
    }

    /// Creates a new [`Punctuation`] with [`Spacing::Joint`] and [`Span::call_site`]
    pub fn new_joint(c: char) -> Self {
        Punctuation::new_spaced(c, Spacing::Joint)
    }
}
