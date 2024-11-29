use crate::ast::PathExprSegment;

impl<'a> PathExprSegment<'a> {
    pub fn into_static(self) -> PathExprSegment<'static> {
        PathExprSegment {
            ident: self.ident,
            generic_args: self
                .generic_args
                .map(|(separator, generic_args)| (separator, generic_args.into_static())),
        }
    }
}
