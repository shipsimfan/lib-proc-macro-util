/// Creates a derive procedural macro named `name` for the function at `path`
///
/// `path` must have the following signature:
/// ```rust
/// fn example(item: proc_macro_util::DeriveItem) -> proc_macro_util::Result<T>
/// where
///    T: proc_macro_util::ToTokens;
/// ```
#[macro_export]
macro_rules! proc_macro_derive {
    (
        $(#[$meta: meta])*
        $name: ident -> $path: path
    ) => {
        #[proc_macro_derive($name)]
        #[allow(non_snake_case)]
        $(#[$meta])*
        pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let token_buffer = $crate::collect_token_stream(input);
            let derive_item = match $crate::parse::<$crate::ast::DeriveItem>(&token_buffer, true, $crate::Span::call_site().end()) {
                Ok(derive_item) => derive_item,
                Err(error) => {
                    error.emit();
                    ::proc_macro::TokenStream::new()
                }
            };

            match $path(derive_item) {
                Ok(output) => $crate::generate(output),
                Err(error) => {
                    error.emit();
                    ::proc_macro::TokenStream::new()
                }
            }
        }
    };

    (
        $(#[$meta: meta])*
        $name: ident ($($attribute: ident),+) -> $path: path
    ) => {
        #[proc_macro_derive($name, attributes($($attribute),+))]
        #[allow(non_snake_case)]
        $(#[$meta])*
        pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let token_buffer = $crate::collect_token_stream(input);
            let derive_item = match $crate::parse::<$crate::ast::DeriveItem>(&token_buffer, true, $crate::Span::call_site().end()) {
                Ok(derive_item) => derive_item,
                Err(error) => {
                    error.emit();
                    return ::proc_macro::TokenStream::new();
                }
            };

            match $path(derive_item) {
                Ok(output) => $crate::generate(output),
                Err(error) => {
                    error.emit();
                    ::proc_macro::TokenStream::new()
                }
            }
        }
    };
}
