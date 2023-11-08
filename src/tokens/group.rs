use crate::{
    parsing::Parser,
    tokens::{Delimiter, Span, TokenBuffer},
    Result,
};
use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Clone)]
pub struct Group<'a> {
    delimiter: Delimiter,
    tokens: &'a TokenBuffer,
    span: Span,
}

impl<'a> Group<'a> {
    pub(crate) fn new(delimiter: Delimiter, tokens: &'a TokenBuffer, span: Span) -> Self {
        Group {
            delimiter,
            tokens,
            span,
        }
    }

    pub fn parse(parser: &mut Parser<'a>, delimiter: Delimiter) -> Result<Self> {
        parser.step(|parser| {
            if let Some(group) = parser.group(delimiter) {
                Ok(group)
            } else {
                Err(parser.error(match delimiter {
                    Delimiter::Brace => "expected square braces",
                    Delimiter::Bracket => "expected curly brackets",
                    Delimiter::Parenthesis => "expected parentheses",
                    Delimiter::None => panic!("Should not be expecting \"None\" delimiters"),
                }))
            }
        })
    }

    pub fn delimiter(&self) -> Delimiter {
        self.delimiter
    }

    pub fn parser(&self) -> Parser<'a> {
        Parser::new(self.tokens)
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

macro_rules! delimiter {
    ($name: ident) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            pub span: Span,
        }

        impl $name {
            pub fn new(span: Span) -> Self {
                $name { span }
            }

            pub fn parse_group<'a>(parser: &mut Parser<'a>) -> Result<(Parser<'a>, Self)> {
                Group::parse(parser, Delimiter::$name)
                    .map(|group| (group.parser(), $name::new(group.span())))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    span: Span::call_site(),
                }
            }
        }

        impl PartialEq for $name {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }

        impl Eq for $name {}

        impl Hash for $name {
            fn hash<H: Hasher>(&self, _: &mut H) {}
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                Display::fmt(self, f)
            }
        }
    };
}

delimiter!(Parenthesis);
delimiter!(Bracket);
delimiter!(Brace);
