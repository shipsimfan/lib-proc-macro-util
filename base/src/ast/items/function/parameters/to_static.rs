use crate::ast::items::FunctionParameters;

impl<'a> FunctionParameters<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> FunctionParameters<'static> {
        match self {
            FunctionParameters::Normal {
                _self,
                first,
                remaining,
                ending,
            } => FunctionParameters::Normal {
                _self: _self.map(|(_self, comma)| (_self.into_static(), comma)),
                first: first.into_static(),
                remaining: remaining
                    .into_iter()
                    .map(|(comma, parameter)| (comma, parameter.into_static()))
                    .collect(),
                ending,
            },
            FunctionParameters::OnlySelf { _self, comma } => FunctionParameters::OnlySelf {
                _self: _self.into_static(),
                comma,
            },
        }
    }
}
