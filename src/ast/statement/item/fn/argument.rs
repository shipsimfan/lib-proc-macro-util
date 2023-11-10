use crate::ast::{PatternType, Receiver};

#[derive(Clone)]
pub enum FunctionArgument {
    Receiver(Receiver),
    Typed(PatternType),
}
