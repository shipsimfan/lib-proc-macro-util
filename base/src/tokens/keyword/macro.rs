macro_rules! keywords {
    [$(
        $(#[$meta: meta])*
        $keyword: literal $name: ident
    )*] => {$(
        $(#[$meta])*
        #[doc = ::std::concat!("`", $keyword, "` keyword")]
        #[derive(Debug, Clone)]
        pub struct $name {
            /// The location of this span
            pub span: Span,
        }

        #[doc = ::std::concat!("Creates a new [`struct@", ::std::stringify!($name), "`] keyword with [`Span::call_site`]")]
        #[allow(non_snake_case)]
        pub fn $name() -> $name {
            $name::new()
        }

        impl $name {
            #[doc = ::std::concat!("Creates a new [`struct@", ::std::stringify!($name), "`] keyword with `span`")]
            pub const fn new_at(span: Span) -> Self {
                $name { span }
            }

            #[doc = ::std::concat!("Creates a new [`struct@", ::std::stringify!($name), "`] keyword with [`Span::call_site`]")]
            pub fn new() -> Self {
                $name::new_at(Span::call_site())
            }

            #[doc = ::std::concat!("Gets the keyword as a [`str`]")]
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
                if let Some(TokenTree::Identifier(identifier)) = parser.next() {
                    if identifier == $keyword {
                        return Ok(Self::new_at(identifier.span()))
                    }
                }

                Err(parser.error(concat!("expected \"", $keyword, "\"")))
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
    )*};
}
pub(super) use keywords;
