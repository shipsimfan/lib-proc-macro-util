macro_rules! punctuation {
    [$($literal: literal $name: ident),*] => {$(
        #[allow(missing_docs)]
        pub struct $name {
            spans: [::proc_macro::Span; Self::LEN],
        }

        impl $name {
            #[allow(missing_docs)]
            pub const LEN: usize = $literal.len();

            #[allow(missing_docs)]
            pub fn new(spans: [::proc_macro::Span; Self::LEN]) -> Self {
                $name {
                    spans
                }
            }
        }

        impl $crate::Parse for $name {
            fn parse(parser: &mut $crate::Parser) -> $crate::Result<Self> {
                let mut spans = [parser.span(); Self::LEN];

                parser.step(|parser| {
                    for (i, c) in $literal.chars().enumerate() {
                        match parser.punctuation() {
                            Some(punctuation) => {
                                spans[i] = punctuation.span();
                                if punctuation.as_char() != c {
                                    break;
                                } else if i == Self::LEN - 1 {
                                    return Ok($name { spans });
                                } else if punctuation.spacing() != ::proc_macro::Spacing::Joint {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }

                    Err($crate::Error::new_at(concat!("expected '", $literal, "'"), spans[0]))
                })
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(&self, generator: &mut $crate::Generator) {
                for (i, c) in $literal.chars().enumerate() {
                    let spacing = if i < Self::LEN - 1 {
                        ::proc_macro::Spacing::Joint
                    } else {
                        ::proc_macro::Spacing::Alone
                    };

                    generator.punctuation($crate::tokens::Punctuation::new(c, spacing, self.spans[i]));
                }
            }
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::new([::proc_macro::Span::call_site(); Self::LEN])
            }
        }
    )*};
}

punctuation![
    ":" Colon,
    "::" DoubleColon,
    "," Comma,
    "." Dot,
    "=" Equals,
    "!" Exclamation,
    ";" SemiColon
];
