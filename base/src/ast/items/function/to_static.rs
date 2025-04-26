use crate::ast::{
    items::{Function, FunctionParameters, FunctionReturnType},
    GenericParams, WhereClause,
};
use std::borrow::Cow;

impl<'a> Function<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Function<'static> {
        Function {
            qualifiers: self.qualifiers.into_static(),
            r#fn: self.r#fn,
            name: Cow::Owned(match self.name {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.clone(),
            }),
            generic_params: self.generic_params.map(GenericParams::into_static),
            parameters: self.parameters.map(FunctionParameters::into_static),
            return_type: self.return_type.map(FunctionReturnType::into_static),
            where_clause: self.where_clause.map(WhereClause::into_static),
            body: self.body.into_static(),
        }
    }
}
