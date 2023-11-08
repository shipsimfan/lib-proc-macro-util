use crate::tokens::{Delimiter, Group, Ident, Literal, Punctuation, Span, TokenStream, TokenTree};

pub(crate) enum TokenTreeBuffer {
    Ident(Ident),
    Punctuation(Punctuation),
    Literal(Literal),
    Group(GroupBuffer),
}

pub(crate) struct TokenBuffer {
    tokens: Box<[TokenTreeBuffer]>,
}

pub(crate) struct GroupBuffer {
    pub delimiter: Delimiter,
    pub buffer: TokenBuffer,
    pub span: Span,
}

impl TokenBuffer {
    pub(crate) fn new(input: TokenStream) -> Self {
        let mut tokens = Vec::new();

        TokenBuffer::append_tokens(&mut tokens, input);

        TokenBuffer {
            tokens: tokens.into_boxed_slice(),
        }
    }

    pub(crate) fn as_slice(&self) -> &[TokenTreeBuffer] {
        &self.tokens
    }

    fn append_tokens(tokens: &mut Vec<TokenTreeBuffer>, input: TokenStream) {
        for token in input {
            if let proc_macro::TokenTree::Group(group) = &token {
                if group.delimiter() == Delimiter::None {
                    TokenBuffer::append_tokens(tokens, group.stream());
                    continue;
                }
            }

            tokens.push(token.into());
        }
    }
}

impl<'a> Into<TokenTree<'a>> for &'a TokenTreeBuffer {
    fn into(self) -> TokenTree<'a> {
        match self {
            TokenTreeBuffer::Ident(ident) => TokenTree::Ident(ident.clone()),
            TokenTreeBuffer::Punctuation(punctuation) => {
                TokenTree::Punctuation(punctuation.clone())
            }
            TokenTreeBuffer::Literal(literal) => TokenTree::Literal(literal.clone()),
            TokenTreeBuffer::Group(group) => TokenTree::Group(group.into()),
        }
    }
}

impl From<proc_macro::TokenTree> for TokenTreeBuffer {
    fn from(token: proc_macro::TokenTree) -> Self {
        match token {
            proc_macro::TokenTree::Ident(ident) => TokenTreeBuffer::Ident(ident),
            proc_macro::TokenTree::Punct(punctuation) => TokenTreeBuffer::Punctuation(punctuation),
            proc_macro::TokenTree::Literal(literal) => TokenTreeBuffer::Literal(literal),
            proc_macro::TokenTree::Group(group) => TokenTreeBuffer::Group(group.into()),
        }
    }
}

impl<'a> Into<Group<'a>> for &'a GroupBuffer {
    fn into(self) -> Group<'a> {
        Group::new(self.delimiter, &self.buffer, self.span)
    }
}

impl From<proc_macro::Group> for GroupBuffer {
    fn from(group: proc_macro::Group) -> Self {
        GroupBuffer {
            delimiter: group.delimiter(),
            buffer: TokenBuffer::new(group.stream()),
            span: group.span(),
        }
    }
}
