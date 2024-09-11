use crate::{Generator, ToTokens};

impl<T: ToTokens, const N: usize> ToTokens for [T; N] {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}

impl<T: ToTokens> ToTokens for std::collections::BTreeSet<T> {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}

impl<T: ToTokens> ToTokens for std::collections::BinaryHeap<T> {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}

impl<T: ToTokens> ToTokens for std::collections::HashSet<T> {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}

impl<T: ToTokens> ToTokens for std::collections::LinkedList<T> {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}

impl<T: ToTokens> ToTokens for std::collections::VecDeque<T> {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self {
            token.to_tokens(generator);
        }
    }
}
