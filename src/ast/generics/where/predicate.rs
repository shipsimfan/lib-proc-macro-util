use crate::ast::{PredicateLifetime, PredicateType};

#[derive(Clone)]
pub enum WherePredicate {
    Lifetime(PredicateLifetime),
    Type(PredicateType),
}
