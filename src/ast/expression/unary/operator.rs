use crate::tokens::{Asterick, Dash};

#[derive(Clone)]
pub enum UnaryOperator {
    Dereference(Asterick),
    Not(Asterick),
    Negate(Dash),
}
