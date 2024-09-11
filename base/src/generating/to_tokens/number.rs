use crate::{tokens::Literal, Generator, ToTokens};

impl ToTokens for i8 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for i16 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for i32 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for i64 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for i128 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for isize {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for u8 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for u16 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for u32 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for u64 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for u128 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for usize {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for f32 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for f64 {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}
