macro_rules! punctuation {
    [$(
        $(#[$meta: meta])*
        $punctuation: literal $name: ident
    )*] => {$(
        $(#[$meta])*
        #[doc = concat!("`", $punctuation, "` punctuation")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            /// The locations of the punctuations
            pub spans: [Span; Self::LEN],

            /// The spacing of the last punctuation
            pub final_spacing: Spacing,
        }


        #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] punctuation with [`Span::call_site`] and a final spacing of [`Spacing::Alone`]")]
        #[allow(non_snake_case)]
        pub fn $name() -> $name {
            $name::new_alone()
        }

        impl $name {
            #[doc = concat!("The number of punctuations that make up `", $punctuation, "`")]
            pub const LEN: usize = $punctuation.len();

            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] punctuation with `spans` and a final spacing of `final_spacing`")]
            pub const fn new_at(spans: [Span; Self::LEN], final_spacing: Spacing) -> Self {
                $name { spans, final_spacing }
            }

            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] punctuation with `spans` and a final spacing of [`Spacing::Alone`]")]
            pub fn new_at_alone(spans: [Span; Self::LEN]) -> Self {
                $name::new_at(spans, Spacing::Alone)
            }

            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] punctuation with `spans` and a final spacing of [`Spacing::Joint`]")]
            pub fn new_at_joint(spans: [Span; Self::LEN]) -> Self {
                $name::new_at(spans, Spacing::Joint)
            }

            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] punctuation with [`Span::call_site`] and a final spacing of `final_spacing`")]
            pub fn new(final_spacing: Spacing) -> Self {
                $name::new_at([Span::call_site(); Self::LEN], final_spacing)
            }

            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] punctuation with [`Span::call_site`] and a final spacing of [`Spacing::Alone`]")]
            pub fn new_alone() -> Self {
                $name::new(Spacing::Alone)
            }

            #[doc = concat!("Creates a new [`struct@", stringify!($name), "`] punctuation with [`Span::call_site`] and a final spacing of [`Spacing::Joint`]")]
            pub fn new_joint() -> Self {
                $name::new(Spacing::Joint)
            }

            #[doc = concat!("Gets the string representation of `", $punctuation, "`")]
            pub const fn as_str(&self) -> &'static str {
                $punctuation
            }
        }

        impl<'a> Parse<'a> for $name {
            fn parse(parser: &mut Parser<'a>) -> Result<Self> {
                let mut spans = [parser.span(); Self::LEN];
                let mut final_spacing = Spacing::Alone;
                for (i, c) in $punctuation.chars().enumerate() {
                    let punctuation = match parser.next() {
                        Some(TokenTree::Punctuation(punctuation)) => punctuation,
                        Some(token_tree) => {
                            return Err(token_tree.span().error(concat!("expected `", $punctuation, "`")));
                        }
                        None => {
                            return Err(parser.span().error(concat!("expected `", $punctuation, "`")))
                        }
                    };

                    if punctuation.as_char() != c || (
                        i < Self::LEN - 1 && punctuation.spacing() != Spacing::Joint
                    ) {
                        return Err(punctuation.span().error(concat!("expected `", $punctuation, "`")));
                    }

                    spans[i] = punctuation.span();
                    final_spacing = punctuation.spacing();
                }

                Ok($name {
                    spans,
                    final_spacing,
                })
            }
        }

        impl ToTokens for $name {
            fn to_tokens(self, generator: &mut Generator) {
                for (i, c) in $punctuation.chars().enumerate() {
                    let spacing = if i < Self::LEN - 1 {
                        Spacing::Joint
                    } else {
                        self.final_spacing
                    };

                    Punctuation::new_spaced_at(c, spacing, self.spans[i])
                        .to_tokens(generator);
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new_alone()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    )*};
}
pub(super) use punctuation;
