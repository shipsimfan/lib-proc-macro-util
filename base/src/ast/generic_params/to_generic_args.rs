use crate::ast::{GenericArgs, GenericParams};

impl<'a> GenericParams<'a> {
    /// Convert these generic parameters into generic arguments
    pub fn to_generic_args(&self) -> GenericArgs<'a> {
        GenericArgs {
            open: self.open,
            args: self
                .params
                .iter()
                .map(|(arg, comma)| (arg.to_generic_arg(), *comma))
                .collect(),
            last_arg: self.last_param.to_generic_arg(),
            last_comma: self.last_comma,
            close: self.close,
        }
    }
}
