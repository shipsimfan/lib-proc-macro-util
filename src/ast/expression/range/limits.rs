use crate::tokens::{DoubleDot, DoubleDotEquals};

#[derive(Clone)]
pub enum RangeLimits {
    HalfOpen(DoubleDot),
    Closed(DoubleDotEquals),
}
