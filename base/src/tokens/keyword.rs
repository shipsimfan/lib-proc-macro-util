macro_rules! keywords {
    [$($literal: literal $name: ident)*] => {$(
        #[allow(missing_docs)]
        #[derive(Debug, Clone)]
        pub struct $name {
            pub span: ::proc_macro::Span,
        }

        #[allow(missing_docs)]
        #[allow(non_snake_case)]
        pub fn $name() -> $name {
            $name::default()
        }

        impl $name {
            #[allow(missing_docs)]
            pub const fn new(span: ::proc_macro::Span) -> Self {
                $name { span }
            }

            #[allow(missing_docs)]
            pub const fn as_str(&self) -> &'static str {
                $literal
            }
        }

        impl<'a> $crate::Parse<'a> for $name {
            fn parse(parser: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
                parser.step(|parser| {
                    if let Some(identifier) = parser.identifier() {
                        if identifier == $literal {
                            return Ok(Self::new(identifier.span()))
                        }
                    }

                    Err(parser.error(concat!("expected \"", $literal, "\"")))
                })
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(&self, generator: &mut $crate::Generator) {
                generator.identifier_string_at($literal, self.span);
            }
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::new(::proc_macro::Span::call_site())
            }
        }
    )*};
}

keywords! [
    "abstract" Abstract
    "as" As
    "async" Async
    "await" Await
    "become" Become
    "box" Box
    "break" Break
    "const" Const
    "continue" Continue
    "crate" Crate
    "do" Do
    "dyn" Dyn
    "else" Else
    "enum" Enum
    "extern" Extern
    "final" Final
    "fn" Fn
    "for" For
    "if" If
    "impl" Impl
    "in" In
    "let" Let
    "loop" Loop
    "macro" Macro
    "match" Match
    "mod" Mod
    "move" Move
    "mut" Mut
    "override" Override
    "priv" Priv
    "pub" Pub
    "ref" Ref
    "return" Return
    "self" LowerSelf
    "Self" UpperSelf
    "static" Static
    "struct" Struct
    "super" Super
    "trait" Trait
    "try" Try
    "type" Type
    "typeof" TypeOf
    "unsafe" Unsafe
    "unsized" Unsized
    "use" Use
    "virtual" Virtual
    "where" Where
    "while" While
    "yield" Yield
];
