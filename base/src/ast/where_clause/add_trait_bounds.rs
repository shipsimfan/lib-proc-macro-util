use crate::{
    ast::{
        GenericParam, GenericParamKind, GenericParams, Type, TypeParamBounds, WhereClause,
        WhereClauseItem,
    },
    Token,
};

impl<'a> WhereClause<'a> {
    /// Adds `bounds` for each type in `generics` to this where clause
    pub fn add_bounds(&mut self, generics: &GenericParams<'a>, bounds: &TypeParamBounds<'a>) {
        for (param, _) in &generics.params {
            self.add_trait_bound(param, bounds);
        }

        self.add_trait_bound(&generics.last_param, bounds);
    }

    /// Adds a bound for `trait` for `param` if it is a  to this where clause
    fn add_trait_bound(&mut self, param: &GenericParam<'a>, bounds: &TypeParamBounds<'a>) {
        let identifier = match &param.kind {
            GenericParamKind::Type(r#type) => r#type.identifier.clone(),
            _ => return,
        };

        if let Some(r#final) = self.r#final.take() {
            self.initial.push((r#final, Token![,]()));
        }

        self.r#final = Some(WhereClauseItem::Type {
            for_lifetimes: None,
            r#type: Type::from_ident(identifier),
            colon: Token![:](),
            bounds: Some(bounds.clone()),
        })
    }
}
