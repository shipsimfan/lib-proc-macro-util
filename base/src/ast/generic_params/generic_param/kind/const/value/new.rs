use crate::ast::ConstParamValue;

impl<'a> ConstParamValue<'a> {
    /// Creates a new [`ConstParamValue`]
    pub fn new<T: Into<ConstParamValue<'a>>>(value: T) -> Self {
        value.into()
    }
}
