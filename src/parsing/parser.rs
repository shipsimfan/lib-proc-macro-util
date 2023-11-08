use super::Parse;
use crate::{
    tokens::{Delimiter, Group, Ident, Punctuation, Span, TokenBuffer, TokenTree, TokenTreeBuffer},
    Error, Result,
};

#[derive(Clone)]
pub struct Parser<'a> {
    input: &'a [TokenTreeBuffer],
    index: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(input: &'a TokenBuffer) -> Self {
        Parser {
            input: input.as_slice(),
            index: 0,
        }
    }

    pub fn current(&self) -> Option<TokenTree<'a>> {
        self.input
            .get(self.index)
            .map(|token_tree| token_tree.into())
    }

    pub fn span(&self) -> Span {
        match self.current() {
            Some(current) => match current {
                TokenTree::Group(group) => group.span(),
                TokenTree::Ident(ident) => ident.span(),
                TokenTree::Literal(literal) => literal.span(),
                TokenTree::Punctuation(punctuation) => punctuation.span(),
            },
            None => Span::call_site(),
        }
    }

    pub fn error<T: std::fmt::Display>(&self, message: T) -> Error {
        Error::new_at(self.span(), message)
    }

    #[allow(unused_variables)]
    pub fn peek<T: Parse<'a>>(&self, token: T) -> bool {
        self.clone().parse::<T>().is_ok()
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
        T::parse(self)
    }

    pub fn ident(&mut self) -> Option<Ident> {
        match self.current()? {
            TokenTree::Ident(ident) => {
                self.advance();
                Some(ident)
            }
            _ => None,
        }
    }

    pub fn punctuation(&mut self) -> Option<Punctuation> {
        match self.current()? {
            TokenTree::Punctuation(punctuation) => Some(punctuation),
            _ => None,
        }
    }

    pub fn group(&mut self, delimiter: Delimiter) -> Option<Group<'a>> {
        let group = match self.current()? {
            TokenTree::Group(group) => group,
            _ => return None,
        };

        if group.delimiter() != delimiter {
            None
        } else {
            Some(group)
        }
    }

    pub fn step<T, F: FnOnce(&mut Parser<'a>) -> Result<T>>(&mut self, f: F) -> Result<T> {
        let mut parser = self.clone();
        f(&mut parser).map(|ret| {
            *self = parser;
            ret
        })
    }
}
