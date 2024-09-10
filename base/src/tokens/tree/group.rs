use crate::{parsing::Parser, tokens::TokenTree, Delimiter, Parse, Result, Span, ToTokens};

/// A delimited group of tokens
#[derive(Debug, Clone)]
pub struct Group {
    /// The span which covers this group
    pub span: Span,

    /// The delimiter dividing the group
    pub delimiter: Delimiter,

    /// The tokens contained by this buffer
    pub tokens: Vec<TokenTree>,
}

impl Group {
    pub const fn new_at(delimiter: Delimiter, span: Span) -> Self {
        Group {
            span,
            delimiter,
            tokens: Vec::new(),
        }
    }

    pub fn parser(&self) -> Parser<'a> {
        self.tokens.into()
    }
}

impl<'a> Parse<'a> for &'a Group<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.group().ok_or(parser.error("expected a group"))
    }
}

impl<'a> Parse<'a> for Group<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Group>().map(|group| group.clone())
    }
}

impl<'a> ToTokens for Group<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        generator
            .group_at(self.delimiter, self.span)
            .generate(&self.tokens);
    }
}

impl<'a> From<proc_macro::Group> for Group<'a> {
    fn from(group: proc_macro::Group) -> Self {
        Group {
            span: group.span(),
            delimiter: group.delimiter(),
            tokens: group.stream().into(),
        }
    }
}

impl<'a> Into<proc_macro::Group> for Group<'a> {
    fn into(self) -> proc_macro::Group {
        proc_macro::Group::new(self.delimiter, self.tokens.into())
    }
}
