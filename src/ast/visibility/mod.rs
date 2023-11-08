use crate::tokens::Pub;

mod restricted;

pub use restricted::VisibilityRestricted;

#[derive(Clone)]
pub enum Visibility {
    Public(Pub),
    Restricted(VisibilityRestricted),
    Inherited,
}
