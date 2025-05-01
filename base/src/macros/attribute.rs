/// Creates a attirbute procedural macro named `name` for the function at `path`
///
/// `path` must have the following signature:
/// ```rust
/// fn example(attr: A, item: proc_macro_util::Item) -> proc_macro_util::Result<T>
/// where
///    A: proc_macro_util::Parse,
///    T: proc_macro_util::ToTokens;
/// ```
#[macro_export]
macro_rules! proc_macro_attribute {
    (
        $(#[$meta: meta])*
        $name: ident -> $path: path
    ) => {
        #[proc_macro_attribute]
        #[allow(non_snake_case)]
        $(#[$meta])*
        pub fn $name(attr_stream: ::proc_macro::TokenStream, item_stream: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let attr_buffer = $crate::collect_token_stream(attr_stream);
            let attr = match $crate::parse(&attr_buffer, true) {
                Ok(attr) => attr,
                Err(error) => return $crate::generate(error),
            };

            let item_buffer = $crate::collect_token_stream(item_stream);
            let item = match $crate::parse::<$crate::ast::Item>(&item_buffer, true) {
                Ok(item) => item,
                Err(error) => return $crate::generate(error),
            };

            match $path(item, attr) {
                Ok(output) => $crate::generate(output),
                Err(error) => $crate::generate(error),
            }
        }
    };
}
