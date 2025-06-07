use crate::ast::{GenericArg, GenericParam};

impl<'a> GenericParam<'a> {
    /// Convert this generic parameter into a generic argument
    pub fn to_generic_arg(&self) -> GenericArg<'a> {
        self.kind.to_generic_arg()
    }
}
