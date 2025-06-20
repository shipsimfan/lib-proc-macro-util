use crate::ast::GenericArgs;

impl<'a> GenericArgs<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GenericArgs<'static> {
        GenericArgs {
            open: self.open,
            args: self
                .args
                .into_iter()
                .map(|(generic_arg, separator)| (generic_arg.into_static(), separator))
                .collect(),
            last_arg: self.last_arg.into_static(),
            last_comma: self.last_comma,
            close: self.close,
        }
    }
}
