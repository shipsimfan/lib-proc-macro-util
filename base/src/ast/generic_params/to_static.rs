use crate::ast::GenericParams;

impl<'a> GenericParams<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GenericParams<'static> {
        GenericParams {
            open: self.open,
            params: self
                .params
                .into_iter()
                .map(|(param, separator)| (param.into_static(), separator))
                .collect(),
            last_param: self.last_param.into_static(),
            last_comma: self.last_comma,
            close: self.close,
        }
    }
}
