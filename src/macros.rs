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
    ($name: ident::$type_name: ident) => {
        mod $name;

        #[proc_macro]
        pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            match $name::$type_name::parse(input) {
                Ok(output) => $crate::generate(output),
                Err(error) => $crate::generate(error),
            }
        }
    };
}
