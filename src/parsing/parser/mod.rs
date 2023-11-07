use super::Parse;
use crate::{Error, Result};
use buffer::TokenBuffer;
use proc_macro::{Ident, Span, TokenStream, TokenTree};

mod buffer;

pub struct Parser<'a> {
    scope: Span,
    input: &'a [TokenTree],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn parse<T: Parse>(input: TokenStream) -> Result<T> {
        let scope = Span::call_site();
        let buffer = TokenBuffer::new(input);

        let mut parser = Parser {
            scope,
            input: buffer.as_slice(),
            index: 0,
        };

        T::parse(&mut parser)
    }

    pub fn error<T: std::fmt::Display>(&self, message: T) -> Error {
        Error::new_at(self.span().unwrap_or(self.scope), message)
    }

    pub fn span(&self) -> Option<Span> {
        Some(match self.current()? {
            TokenTree::Group(group) => group.span(),
            TokenTree::Ident(ident) => ident.span(),
            TokenTree::Literal(literal) => literal.span(),
            TokenTree::Punct(punctuation) => punctuation.span(),
        })
    }

    pub fn ident(&mut self) -> Option<Ident> {
        match self.current()? {
            TokenTree::Ident(ident) => {
                let ret = ident.clone();
                self.advance();
                Some(ret)
            }
            _ => None,
        }
    }

    fn current(&self) -> Option<&TokenTree> {
        self.input.get(self.index)
    }

    fn advance(&mut self) {
        self.index += 1;
    }
}
