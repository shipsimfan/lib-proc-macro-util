use crate::tokens::Question;

#[derive(Clone)]
pub enum TraitBoundModifier {
    None,
    Maybe(Question),
}
