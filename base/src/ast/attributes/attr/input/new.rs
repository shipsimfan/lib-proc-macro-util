use crate::ast::AttrInput;

impl<'a> AttrInput<'a> {
    /// Creates a new [`AttrInput`] from `value`
    pub fn new<T: Into<AttrInput<'a>>>(value: T) -> Self {
        value.into()
    }
}
