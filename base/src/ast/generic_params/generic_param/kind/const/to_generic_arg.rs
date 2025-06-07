use crate::ast::{ConstParam, GenericArgsConst, SimplePathSegment};

impl<'a> ConstParam<'a> {
    /// Convert this generic parameter into a generic argument
    pub fn to_generic_arg(&self) -> GenericArgsConst<'a> {
        GenericArgsConst::SimplePathSegment(SimplePathSegment::Identifier(self.identifier.clone()))
    }
}
