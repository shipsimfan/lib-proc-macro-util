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
            pub fn new(span: ::proc_macro::Span) -> Self {
                $name { span }
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
    "async" Async
    "await" Await
    "const" Const
    "crate" Crate
    "dyn" Dyn
    "fn" Fn
    "let" Let
    "mut" Mut
    "pub" Pub
    "self" LowerSelf
    "Self" UpperSelf
    "struct" Struct
    "super" Super
    "unsafe" Unsafe
];
