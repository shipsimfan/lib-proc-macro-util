use crate::{collect_token_stream, tokens::Group, Delimiter, Span};

impl Group {
    /// Creates a new [`Group`] from `group`
    pub fn new_raw(group: proc_macro::Group) -> Self {
        Group {
            span: group.span(),
            delimiter: group.delimiter(),
            tokens: collect_token_stream(group.stream()),
        }
    }

    /// Creates a new [`Group`] with `span`
    pub const fn new_at(delimiter: Delimiter, span: Span) -> Self {
        Group {
            span,
            delimiter,
            tokens: Vec::new(),
        }
    }

    /// Creates a new [`Group`] with [`Span::call_site`]
    pub fn new(delimiter: Delimiter) -> Self {
        Group {
            span: Span::call_site(),
            delimiter,
            tokens: Vec::new(),
        }
    }

    /// Creates a new [`Group`] with [`Span::call_site`] and [`Delimiter::Brace`]
    pub fn new_brace() -> Self {
        Group {
            span: Span::call_site(),
            delimiter: Delimiter::Brace,
            tokens: Vec::new(),
        }
    }

    /// Creates a new [`Group`] with [`Span::call_site`] and [`Delimiter::Bracket`]
    pub fn new_bracket() -> Self {
        Group {
            span: Span::call_site(),
            delimiter: Delimiter::Bracket,
            tokens: Vec::new(),
        }
    }

    /// Creates a new [`Group`] with [`Span::call_site`] and [`Delimiter::Parenthesis`]
    pub fn new_parenthesis() -> Self {
        Group {
            span: Span::call_site(),
            delimiter: Delimiter::Parenthesis,
            tokens: Vec::new(),
        }
    }
}
