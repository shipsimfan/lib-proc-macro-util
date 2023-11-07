use crate::{parsing::Parser, Result};
use proc_macro::Span;

macro_rules! keyword {
    ($name: ident $str: literal) => {
        pub struct $name {
            pub span: Span,
        }

        #[allow(non_snake_case)]
        pub fn $name(span: Span) -> $name {
            $name { span }
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    span: Span::call_site(),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str($str)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }

        impl Eq for $name {}

        impl $crate::parsing::Parse for $name {
            fn parse(parser: &mut Parser) -> Result<Self> {
                Ok($name {
                    span: keyword(parser, $str)?,
                })
            }
        }
    };
}

fn keyword(parser: &mut Parser, keyword: &str) -> Result<Span> {
    if let Some(ident) = parser.ident() {
        if ident.to_string() == keyword {
            return Ok(ident.span());
        }
    }

    Err(parser.error(format_args!("expected keyword \"{}\"", keyword)))
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
keyword!(Return "return");
keyword!(SelfType "selftype");
keyword!(SelfValue "selfvalue");
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
