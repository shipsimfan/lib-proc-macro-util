use crate::{
    parsing::{Parse, Parser},
    Result,
};
use proc_macro::Span;
use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

macro_rules! keyword {
    ($name: ident $str: literal) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            pub span: Span,
        }

        impl $name {
            pub fn new(span: Span) -> Self {
                $name { span }
            }
        }

        impl std::default::Default for $name {
            fn default() -> Self {
                $name {
                    span: Span::call_site(),
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str($str)
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

        impl<'a> Parse<'a> for $name {
            fn parse(parser: &mut Parser<'a>) -> Result<Self> {
                Ok($name {
                    span: keyword(parser, $str)?,
                })
            }
        }
    };
}

fn keyword(parser: &mut Parser, keyword: &str) -> Result<Span> {
    parser.step(|parser| {
        if let Some(ident) = parser.ident() {
            if ident.to_string() == keyword {
                return Ok(ident.span());
            }
        }

        Err(parser.error(format_args!("expected keyword \"{}\"", keyword)))
    })
}

keyword!(Abstract "abstract");
keyword!(As "as");
keyword!(Async "async");
keyword!(Auto "auto");
keyword!(Await "await");
keyword!(Become "become");
keyword!(Box "box");
keyword!(Break "break");
keyword!(Const "const");
keyword!(Continue "continue");
keyword!(Crate "crate");
keyword!(Default "default");
keyword!(Do "do");
keyword!(Dyn "dyn");
keyword!(Else "else");
keyword!(Enum "enum");
keyword!(Extern "extern");
keyword!(Final "final");
keyword!(Fn "fn");
keyword!(For "for");
keyword!(If "if");
keyword!(Impl "impl");
keyword!(In "in");
keyword!(Let "let");
keyword!(Loop "loop");
keyword!(Macro "macro");
keyword!(Match "match");
keyword!(Mod "mod");
keyword!(Move "move");
keyword!(Mut "mut");
keyword!(Override "override");
keyword!(Priv "priv");
keyword!(Pub "pub");
keyword!(Ref "ref");
keyword!(Return "return");
keyword!(SelfType "Self");
keyword!(SelfValue "self");
keyword!(Static "static");
keyword!(Super "super");
keyword!(Trait "trait");
keyword!(Try "try");
keyword!(Type "type");
keyword!(Typeof "typeof");
keyword!(Union "union");
keyword!(Unsafe "unsafe");
keyword!(Unsized "unsized");
keyword!(Use "use");
keyword!(Virtual "virtual");
keyword!(Where "where");
keyword!(While "while");
keyword!(Yield "yield");
