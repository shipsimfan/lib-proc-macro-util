use crate::tokens::Type;

#[derive(Clone)]
pub struct TypeGroup {
    pub group: proc_macro::Group,
    pub element: Box<Type>,
}
