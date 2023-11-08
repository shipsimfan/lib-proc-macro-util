use crate::tokens::{RightArrow, Type};

#[derive(Clone)]
pub enum ReturnType {
    Default,
    Type(RightArrow, Type),
}
