use crate::{
    collect_token_stream, into_token_stream, parsing::Parser, tokens::TokenTree, Delimiter, Parse,
    Result, Span, ToTokens,
};

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

    pub fn parser(&self) -> Parser {
        Parser::new(&self.tokens)
    }
}

impl<'a> Parse<'a> for &'a Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Group(group)) => Ok(group),
            _ => Err(parser.error("expected a group")),
        }
    }
}

impl<'a> Parse<'a> for Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Group>().map(|group| group.clone())
    }
}

impl<'a> ToTokens for Group {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        generator.push(TokenTree::Group(self.clone()));
    }
}

impl<'a> From<proc_macro::Group> for Group {
    fn from(group: proc_macro::Group) -> Self {
        Group {
            span: group.span(),
            delimiter: group.delimiter(),
            tokens: collect_token_stream(group.stream()),
        }
    }
}

impl<'a> Into<proc_macro::Group> for Group {
    fn into(self) -> proc_macro::Group {
        proc_macro::Group::new(self.delimiter, into_token_stream(self.tokens))
    }
}
