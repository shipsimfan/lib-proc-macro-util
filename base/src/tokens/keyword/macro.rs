macro_rules! keywords {
    [$(
        $(#[$meta: meta])*
        $keyword: literal $name: ident
    )*] => {$(
        $(#[$meta])*
        #[doc = concat!("`", $keyword, "` keyword")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            /// The location of this keyword
            pub span: Span,
        }

        #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] keyword with [`Span::call_site`]")]
        #[allow(non_snake_case)]
        pub fn $name() -> $name {
            $name::new()
        }

        impl $name {
            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] keyword with `span`")]
            pub const fn new_at(span: Span) -> Self {
                $name { span }
            }

            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] keyword with [`Span::call_site`]")]
            pub fn new() -> Self {
                $name::new_at(Span::call_site())
            }

            #[doc = concat!("Gets the keyword as a [`str`]")]
            pub const fn as_str(&self) -> &'static str {
                $keyword
            }
        }

        impl Into<Identifier> for $name {
            fn into(self) -> Identifier {
                Identifier::new_at($keyword, self.span)
            }
        }

        impl Into<TokenTree> for $name {
            fn into(self) -> TokenTree {
                TokenTree::Identifier(self.into())
            }
        }

        impl<'a> Parse<'a> for $name {
            fn parse(parser: &mut Parser<'a>) -> Result<Self> {
                let span = match parser.next() {
                    Some(TokenTree::Identifier(identifier)) => {
                        if identifier == $keyword {
                            return Ok(Self::new_at(identifier.span()))
                        }

                        identifier.span()
                    }
                    Some(token_tree) => token_tree.span(),
                    None => parser.span(),
                };

                Err(span.error(concat!("expected `", $keyword, "`")))
            }
        }

        impl ToTokens for $name {
            fn to_tokens(self, generator: &mut Generator) {
                generator.push(self.into());
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    )*};
}
pub(super) use keywords;
