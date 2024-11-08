use crate::{
    into_token_stream,
    tokens::{Group, TokenTree},
};
use std::borrow::Cow;

impl Into<proc_macro::Group> for Group {
    fn into(self) -> proc_macro::Group {
        proc_macro::Group::new(self.delimiter, into_token_stream(self.tokens))
    }
}

impl Into<TokenTree> for Group {
    fn into(self) -> TokenTree {
        TokenTree::Group(self)
    }
}

impl<'a> Into<Cow<'a, Group>> for Group {
    fn into(self) -> Cow<'a, Group> {
        Cow::Owned(self)
    }
}

impl<'a> Into<Cow<'a, Group>> for &'a Group {
    fn into(self) -> Cow<'a, Group> {
        Cow::Borrowed(self)
    }
}
