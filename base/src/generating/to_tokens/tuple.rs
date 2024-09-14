use crate::{Generator, ToTokens};

impl<T1: ToTokens, T2: ToTokens> ToTokens for (T1, T2) {
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
        self.1.to_tokens(generator);
    }
}

impl<T1: ToTokens, T2: ToTokens, T3: ToTokens> ToTokens for (T1, T2, T3) {
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
        self.1.to_tokens(generator);
        self.2.to_tokens(generator);
    }
}

impl<T1: ToTokens, T2: ToTokens, T3: ToTokens, T4: ToTokens> ToTokens for (T1, T2, T3, T4) {
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
        self.1.to_tokens(generator);
        self.2.to_tokens(generator);
        self.3.to_tokens(generator);
    }
}

impl<T1: ToTokens, T2: ToTokens, T3: ToTokens, T4: ToTokens, T5: ToTokens> ToTokens
    for (T1, T2, T3, T4, T5)
{
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
        self.1.to_tokens(generator);
        self.2.to_tokens(generator);
        self.3.to_tokens(generator);
        self.4.to_tokens(generator);
    }
}

impl<T1: ToTokens, T2: ToTokens, T3: ToTokens, T4: ToTokens, T5: ToTokens, T6: ToTokens> ToTokens
    for (T1, T2, T3, T4, T5, T6)
{
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
        self.1.to_tokens(generator);
        self.2.to_tokens(generator);
        self.3.to_tokens(generator);
        self.4.to_tokens(generator);
        self.5.to_tokens(generator);
    }
}

impl<
        T1: ToTokens,
        T2: ToTokens,
        T3: ToTokens,
        T4: ToTokens,
        T5: ToTokens,
        T6: ToTokens,
        T7: ToTokens,
    > ToTokens for (T1, T2, T3, T4, T5, T6, T7)
{
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
        self.1.to_tokens(generator);
        self.2.to_tokens(generator);
        self.3.to_tokens(generator);
        self.4.to_tokens(generator);
        self.5.to_tokens(generator);
        self.6.to_tokens(generator);
    }
}

impl<
        T1: ToTokens,
        T2: ToTokens,
        T3: ToTokens,
        T4: ToTokens,
        T5: ToTokens,
        T6: ToTokens,
        T7: ToTokens,
        T8: ToTokens,
    > ToTokens for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
        self.1.to_tokens(generator);
        self.2.to_tokens(generator);
        self.3.to_tokens(generator);
        self.4.to_tokens(generator);
        self.5.to_tokens(generator);
        self.6.to_tokens(generator);
        self.7.to_tokens(generator);
    }
}
