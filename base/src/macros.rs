/// A utility for creating procedural macro functions
///
/// ## Format
/// `proc_macro_function!($name::$type_name)`
///
/// ## Parameters
///  * `$name` - The name of this procedural macro. This will also be the module holding the type
///              to parse.
///  * `$type_name` - The name of the type to parse
#[macro_export]
macro_rules! proc_macro_function {
    ($(#[$outer: meta])* $name: ident::$type_name: ident) => {
        mod $name;

        #[proc_macro]
        $(#[$outer])*
        pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let token_buffer = input.into();
            match $crate::parse::<$name::$type_name>(&token_buffer, true) {
                Ok(output) => $crate::generate(&output),
                Err(error) => $crate::generate(&error),
            }
        }
    };
}

/// A utility for creating procedural macro derives
///
/// ## Format
/// `proc_macro_derive!($name::$type_name)`
///
/// ## Parameters
///  * `$name` - The name of this procedural macro and the name of the module holding a function  
///              with the signature `generate(Derive) -> Result<T: ToTokens, E: ToTokens>`
///  * `$type_name` - The name of the trait to derive
#[macro_export]
macro_rules! proc_macro_derive {
    ($name: ident::$type_name: ident) => {
        mod $name;

        #[proc_macro_derive($type_name)]
        pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let token_buffer = input.into();
            match $crate::parse::<$name::$type_name>(&token_buffer, true) {
                Ok(output) => $crate::generate(&output),
                Err(error) => $crate::generate(&error),
            }
        }
    };
}
