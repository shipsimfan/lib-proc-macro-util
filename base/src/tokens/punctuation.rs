/// A trait for any type of punctuation
pub trait PunctuationToken: crate::ToTokens {
    /// Gets the string representation of this punctuation
    ///
    /// ## Return Value
    /// Returns the string representation of this punctuation
    fn as_str(&self) -> &'static str;
}

impl crate::ToTokens for Box<dyn PunctuationToken> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.as_ref().to_tokens(generator)
    }
}

macro_rules! one_punctuation_token_impl {
    ($parser: ident, $literal: literal, $name: ident) => {
        if let Ok(punctuation) = $parser.parse::<$name>() {
            return Ok(Box::new(punctuation));
        }
    };
}

macro_rules! punctuation_token_impl {
    ($($literal: literal $name: ident),*) => {
        impl $crate::Parse for Box<dyn PunctuationToken> {
            fn parse(parser: &mut $crate::Parser) -> $crate::Result<Self> {
                $(one_punctuation_token_impl!(parser, $literal, $name);)*

                Err(parser.error("expected a punctuation"))
            }
        }
    };
}

macro_rules! one_punctuation {
    ($literal: literal, $name: ident) => {
        #[allow(missing_docs)]
        #[derive(Clone)]
        pub struct $name {
            spans: [::proc_macro::Span; Self::LEN],
        }

        impl $name {
            #[allow(missing_docs)]
            pub const LEN: usize = $literal.len();

            #[allow(missing_docs)]
            pub fn new(spans: [::proc_macro::Span; Self::LEN]) -> Self {
                $name { spans }
            }
        }

        impl PunctuationToken for $name {
            fn as_str(&self) -> &'static str {
                $literal
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

                    Err($crate::Error::new_at(
                        concat!("expected '", $literal, "'"),
                        spans[0],
                    ))
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

                    generator.punctuation($crate::tokens::Punctuation::new(
                        c,
                        spacing,
                        self.spans[i],
                    ));
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
        punctuation_token_impl!($($literal $name),*);
        $(one_punctuation!($literal, $name);)*
    };
}

punctuation![
    "::" DoubleColon
    "&" Ampersand
    ":" Colon
    "," Comma
    "." Dot
    "=" Equals
    "!" Exclamation
    "#" Hash
    ";" SemiColon
];
