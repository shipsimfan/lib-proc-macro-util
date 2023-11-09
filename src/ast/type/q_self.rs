use crate::tokens::{As, LeftTriangle, RightTriangle, Type};

#[derive(Clone)]
pub struct QSelf {
    pub left_triangle: LeftTriangle,
    pub r#type: Box<Type>,
    pub position: usize,
    pub r#as: Option<As>,
    pub right_triangle: RightTriangle,
}
