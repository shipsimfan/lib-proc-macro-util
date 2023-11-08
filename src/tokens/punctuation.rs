use crate::{parsing::Parser, Error, Result};
use proc_macro::{Spacing, Span};
use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

macro_rules! punctuation {
    ($name: ident $str: literal [$count: literal]) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            pub spans: [Span; $count],
        }

        impl $name {
            pub fn new(spans: [Span; $count]) -> Self {
                $name { spans }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    spans: [Span::call_site(); $count],
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str(concat!("'", $str, "'"))
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                Display::fmt(self, f)
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

        impl<'a> $crate::parsing::Parse<'a> for $name {
            fn parse(parser: &mut Parser) -> Result<Self> {
                Ok($name {
                    spans: punctuation(parser, $str)?,
                })
            }
        }
    };
}

fn punctuation<const N: usize>(parser: &mut Parser, token: &str) -> Result<[Span; N]> {
    assert_eq!(token.len(), N);

    let mut spans = [parser.span(); N];
    parser.step(|parser| {
        for (i, c) in token.chars().enumerate() {
            match parser.punctuation() {
                Some(punct) => {
                    spans[i] = punct.span();

                    if punct.as_char() == c {
                        break;
                    } else if i == token.len() - 1 {
                        return Ok(spans);
                    } else if punct.spacing() != Spacing::Joint {
                        break;
                    }
                }
                None => break,
            }
        }

        Err(Error::new_at(spans[0], format!("expected `{}`", token)))
    })
}

punctuation!(Ampersand "&" [1]);
punctuation!(DoubleAmpersand "&&" [2]);
punctuation!(AmpersandEquals "&=" [2]);
punctuation!(At "@" [1]);
punctuation!(Caret "^" [1]);
punctuation!(CaretEquals "^=" [2]);
punctuation!(Colon ":" [1]);
punctuation!(Comma "," [1]);
punctuation!(Dollar "$" [1]);
punctuation!(Dot "." [1]);
punctuation!(DoubleDot ".." [2]);
punctuation!(TripleDot "..." [3]);
punctuation!(DoubleDotEquals "..=" [3]);
punctuation!(Equals "=" [1]);
punctuation!(DoubleEquals "==" [2]);
punctuation!(FatArrow "=>" [2]);
punctuation!(RightTriangleEquals ">=" [2]);
punctuation!(RightTriangle ">" [1]);
punctuation!(LeftArrow "<-" [2]);
punctuation!(LeftTriangleEquals "<=" [2]);
punctuation!(LeftTriangle "<" [1]);
punctuation!(Dash "-" [1]);
punctuation!(DashEquals "-=" [2]);
punctuation!(ExclamationEquals "!=" [2]);
punctuation!(Exclamation "!" [1]);
punctuation!(VerticalBar "|" [1]);
punctuation!(VerticalBarEquals "|=" [2]);
punctuation!(DoubleVerticalBar "||" [2]);
punctuation!(DoubleColon "::" [2]);
punctuation!(Percent "%" [1]);
punctuation!(PercentEquals "%=" [2]);
punctuation!(Plus "+" [1]);
punctuation!(PlusEquals "+=" [2]);
punctuation!(Pound "#" [1]);
punctuation!(Question "?" [1]);
punctuation!(RightArrow "->" [2]);
punctuation!(SemiColon ";" [1]);
punctuation!(DoubleLeftTriange "<<" [2]);
punctuation!(DoubleLeftTriangeEquals "<<=" [3]);
punctuation!(DoubleRightTriangle ">>" [2]);
punctuation!(DoubleRightTriangleEquals ">>=" [3]);
punctuation!(Slash "/" [1]);
punctuation!(SlashEquals "/=" [2]);
punctuation!(Asterick "*" [1]);
punctuation!(AsterickEquals "*=" [2]);
punctuation!(Tilde "~" [1]);
