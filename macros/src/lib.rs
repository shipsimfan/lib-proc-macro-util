use base::proc_macro_function;

proc_macro_function!(
    /// Generates tokens using a generator for an input
    ///
    /// The format of this macro is as follows:
    /// ```
    /// to_tokens! { generator
    ///     ...
    /// }
    /// ```
    /// where:
    ///  - `generator` is an identifier which is the name of the generator
    ///  - `...` is any tokens to be generated. If an identifier is preceeded by a `#` in this, it
    ///    will generate the tokens for the corresponding variable instead of generating the
    ///    identifier and `#`.
    to_tokens::ToTokensMacro
);
