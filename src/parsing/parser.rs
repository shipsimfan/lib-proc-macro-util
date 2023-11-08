use proc_macro::Spacing;

use super::Parse;
use crate::{
    tokens::{
        Delimiter, Group, Ident, Literal, Punctuation, Span, TokenBuffer, TokenStream, TokenTree,
        TokenTreeBuffer,
    },
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

    pub fn is_empty(&self) -> bool {
        self.index >= self.input.len()
    }

    pub fn error<T: std::fmt::Display>(&self, message: T) -> Error {
        Error::new_at(self.span(), message)
    }

    pub fn peek<T: Parse<'a>>(&self) -> bool {
        self.clone().parse::<T>().is_ok()
    }

    pub fn peek2<T: Parse<'a>>(&self) -> bool {
        let mut peek = self.clone();
        Self::skip(&mut peek);
        peek.peek::<T>()
    }

    pub fn peek3<T: Parse<'a>>(&self) -> bool {
        let mut peek = self.clone();
        Self::skip(&mut peek);
        Self::skip(&mut peek);
        peek.peek::<T>()
    }

    pub fn skip(&mut self) {
        let len = match match self.current() {
            Some(current) => current,
            None => return,
        } {
            TokenTree::Punctuation(punct)
                if punct.as_char() == '\'' && punct.spacing() == Spacing::Joint =>
            {
                self.index += 1;
                let len = if self.peek::<Ident>() { 2 } else { 1 };
                self.index -= 1;
                len
            }
            _ => 1,
        };

        self.index += len;
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
            TokenTree::Punctuation(punctuation) => {
                self.advance();
                Some(punctuation)
            }
            _ => None,
        }
    }

    pub fn literal(&mut self) -> Option<Literal> {
        match self.current()? {
            TokenTree::Literal(literal) => {
                self.advance();
                Some(literal)
            }
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
            self.advance();
            Some(group)
        }
    }

    pub fn delimiter(&mut self) -> Option<(Delimiter, Parser<'a>)> {
        match self.current()? {
            TokenTree::Group(group) => Some((group.delimiter(), group.parser())),
            _ => return None,
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

impl<'a> Into<TokenStream> for Parser<'a> {
    fn into(self) -> TokenStream {
        self.map(|token_tree| -> proc_macro::TokenTree { token_tree.into() })
            .collect()
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = TokenTree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current();
        self.advance();
        ret
    }
}
