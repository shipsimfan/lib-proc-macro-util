use super::OwnedTokenTree;
use crate::parsing::TokenBuffer;
use proc_macro::{Delimiter, Span};

#[derive(Debug)]
pub(crate) struct OwnedGroup {
    pub(crate) span: Span,
    pub(crate) delimiter: Delimiter,
    pub(crate) tokens: TokenBuffer,
}

impl OwnedGroup {
    pub(crate) fn new(span: Span, delimiter: Delimiter, tokens: Vec<OwnedTokenTree>) -> Self {
        OwnedGroup {
            span,
            delimiter,
            tokens: tokens.into(),
        }
    }
}

impl From<proc_macro::Group> for OwnedGroup {
    fn from(group: proc_macro::Group) -> Self {
        OwnedGroup {
            span: group.span(),
            delimiter: group.delimiter(),
            tokens: group.stream().into(),
        }
    }
}

impl Into<proc_macro::Group> for OwnedGroup {
    fn into(self) -> proc_macro::Group {
        let mut group = proc_macro::Group::new(self.delimiter, self.tokens.into());
        group.set_span(self.span);
        group
    }
}
