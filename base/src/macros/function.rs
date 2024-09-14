/// Creates a procedural macro function named `name` for the type at `path`
///
/// The type must implement two traits:
///  * [`Parse`](crate::Parse) - Used to parse the input
///  * [`ToTokens`](crate::ToTokens) - Used to generate the output
///
/// # Example
/// ```
/// proc_macro_function!(
///     /// Some meta data
///     macro_name -> foo::Bar
/// );
/// ```
#[macro_export]
macro_rules! proc_macro_function {
    (
        $(#[$meta: meta])*
        $name: ident -> $path: path
    ) => {
        #[proc_macro]
        $(#[$meta])*
        pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let token_buffer = $crate::collect_token_stream(input);
            match $crate::parse::<$path>(&token_buffer, true) {
                Ok(output) => $crate::generate(output),
                Err(error) => $crate::generate(error),
            }
        }
    };
}
