use crate::{
    ast::{Lifetime, TraitBound},
    tokens::TokenStream,
};

#[derive(Clone)]
pub enum TypeParameterBound {
    Trait(TraitBound),
    Lifetime(Lifetime),
    Verbatime(TokenStream),
}
