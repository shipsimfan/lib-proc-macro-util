use crate::{
    ast::{
        GenericParam, GenericParamKind, GenericParams, TraitBound, Type, TypeParamBound,
        TypeParamBounds, WhereClause, WhereClauseItem,
    },
    Token,
};

impl<'a> WhereClause<'a> {
    /// Adds bounds for `trait` for each type in `generics` to this where clause
    pub fn add_trait_bounds(&mut self, generics: &GenericParams<'a>, r#trait: &TraitBound<'a>) {
        for (param, _) in &generics.params {
            self.add_trait_bound(param, r#trait);
        }

        self.add_trait_bound(&generics.last_param, r#trait);
    }

    /// Adds a bound for `trait` for `param` if it is a  to this where clause
    fn add_trait_bound(&mut self, param: &GenericParam<'a>, r#trait: &TraitBound<'a>) {
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
            bounds: Some(TypeParamBounds {
                first: TypeParamBound::Trait(Box::new(r#trait.clone())),
                remaining: Vec::new(),
                end: None,
            }),
        })
    }
}
