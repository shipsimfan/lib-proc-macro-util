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
    ($(#[$outer: meta])* $name: ident::$type_name: ident) => {
        mod $name;

        #[proc_macro_derive($type_name)]
        $(#[$outer])*
        pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
            let token_buffer = input.into();
            match $name::generate($crate::parse(&token_buffer, true).unwrap()) {
                Ok(output) => $crate::generate(&output),
                Err(error) => $crate::generate(&error),
            }
        }
    };
}

/// A utility for creating attribute procedural macros
///
/// ## Format
/// `proc_macro_attribute!($name)`
///
/// ## Parameters
///  * `$name` - The name of this procedural macro and the name of the module holding a function  
///              with the signature:
///              `generate(T1: ToTokens, Item) -> Result<T2: ToTokens, E: ToTokens>` where `T1` is
///              the parsed from the delimited token tree following the attribute's name, not
///              including the outer delimiters.
#[macro_export]
macro_rules! proc_macro_attribute {
    ($(#[$outer: meta])* $name: ident) => {
        mod $name;

        #[proc_macro_attribute]
        $(#[$outer])*
        pub fn $name(
            attribute_stream: ::proc_macro::TokenStream,
            item_stream: ::proc_macro::TokenStream,
        ) -> ::proc_macro::TokenStream {
            let attribute_buffer = attribute_stream.into();
            let item_buffer = item_stream.into();

            let attribute = match $crate::parse(&attribute_buffer, true) {
                Ok(attribute) => attribute,
                Err(error) => return $crate::generate(&error),
            };

            match $name::generate(attribute, $crate::parse(&item_buffer, true).unwrap()) {
                Ok(output) => $crate::generate(&output),
                Err(error) => $crate::generate(&error),
            }
        }
    };
}
