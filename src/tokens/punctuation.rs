macro_rules! punctuation {
    [$($literal: literal $name: ident),*] => {$(
        #[allow(missing_docs)]
        pub struct $name {
            spans: [::proc_macro::Span; $literal.len()],
        }

        impl $name {
            #[allow(missing_docs)]
            pub fn new(spans: [::proc_macro::Span; $literal.len()]) -> Self {
                $name {
                    spans
                }
            }
        }

        impl $crate::ToTokens for $name {
            fn to_tokens(&self, generator: &mut $crate::Generator) {
                for (i, c) in $literal.chars().enumerate() {
                    let spacing = if i < $literal.len() - 1 {
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
                Self::new([::proc_macro::Span::call_site(); $literal.len()])
            }
        }
    )*};
}

punctuation![
    ":" Colon,
    "::" DoubleColon,
    "," Comma,
    "!" Exclamation,
    ";" SemiColon
];
