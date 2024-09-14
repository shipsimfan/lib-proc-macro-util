use crate::{
    into_token_stream,
    tokens::{Group, TokenTree},
};

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
