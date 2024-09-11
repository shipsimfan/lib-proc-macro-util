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
                    if let Some($crate::tokens::TokenTree::Identifier(identifier)) = parser.next() {
                        if identifier == $literal {
                            return Ok(Self::new(identifier.span()))
                        }
                    }

                    Err(parser.error(concat!("expected \"", $literal, "\"")))
                })
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(self, generator: &mut $crate::Generator) {
                generator.push($crate::tokens::TokenTree::Identifier($crate::tokens::Identifier::new_at($literal, self.span)));
            }
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::new(::proc_macro::Span::call_site())
            }
        }
    )*};
}
pub(super) use keywords;
