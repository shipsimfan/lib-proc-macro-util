macro_rules! one_punctuation {
    ($literal: literal, $name: ident) => {
        #[allow(missing_docs)]
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            spans: [::proc_macro::Span; Self::LEN],
            final_spacing: ::proc_macro::Spacing,
        }

        #[allow(missing_docs)]
        #[allow(non_snake_case)]
        pub fn $name() -> $name {
            $name::default()
        }

        impl $name {
            #[allow(missing_docs)]
            pub const LEN: usize = $literal.len();

            #[allow(missing_docs)]
            pub fn new(spans: [::proc_macro::Span; Self::LEN]) -> Self {
                $name {
                    spans,
                    final_spacing: ::proc_macro::Spacing::Alone,
                }
            }

            #[allow(missing_docs)]
            pub fn new_joint(spans: [::proc_macro::Span; Self::LEN]) -> Self {
                $name {
                    spans,
                    final_spacing: ::proc_macro::Spacing::Joint,
                }
            }

            #[allow(missing_docs)]
            pub const fn as_str(&self) -> &'static str {
                $literal
            }
        }

        impl<'a> $crate::Parse<'a> for $name {
            fn parse(parser: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
                let mut spans = [parser.span(); Self::LEN];

                parser.step(|parser| {
                    for (i, c) in $literal.chars().enumerate() {
                        match parser.next() {
                            Some($crate::tokens::TokenTree::Punctuation(punctuation)) => {
                                spans[i] = punctuation.span();
                                if punctuation.as_char() != c {
                                    break;
                                } else if i == Self::LEN - 1 {
                                    return Ok($name {
                                        spans,
                                        final_spacing: punctuation.spacing(),
                                    });
                                } else if punctuation.spacing() != ::proc_macro::Spacing::Joint {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }

                    Err($crate::Error::new_at(
                        concat!("expected '", $literal, "'"),
                        spans[0],
                    ))
                })
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(self, generator: &mut $crate::Generator) {
                for (i, c) in $literal.chars().enumerate() {
                    let spacing = if i < Self::LEN - 1 {
                        ::proc_macro::Spacing::Joint
                    } else {
                        self.final_spacing
                    };

                    $crate::tokens::Punctuation::new_spaced_at(c, spacing, self.spans[i])
                        .to_tokens(generator);
                }
            }
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self::new([::proc_macro::Span::call_site(); Self::LEN])
            }
        }
    };
}

macro_rules! punctuation {
    [$($literal: literal $name: ident)*] => {
        $(one_punctuation!($literal, $name);)*
    };
}

punctuation![
    "::" DoubleColon
    "==" DoubleEquals
    "->" RightArrow
    "=>" FatRightArrow
    "&" Ampersand
    "'" Apostrophe
    "*" Asterick
    ":" Colon
    "," Comma
    "-" Dash
    "." Dot
    "=" Equals
    "!" Exclamation
    "#" Hash
    "<" LeftTriangle
    "?" QuestionMark
    ">" RightTriangle
    ";" SemiColon
    "_" Underscore
    "|" VerticalBar
    "/" ForwardSlash
    "+" Plus
];
