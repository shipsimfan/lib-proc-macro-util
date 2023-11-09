use crate::tokens::TokenStream;

mod array;
mod bare_function;
mod group;
mod impl_trait;
mod infer;
mod r#macro;
mod never;
mod parentheses;
mod path;
mod pointer;
mod q_self;
mod reference;
mod r#return;
mod slice;
mod trait_object;
mod tuple;

pub use array::TypeArray;
pub use bare_function::{BareFunctionArgument, BareVariadic, TypeBareFunction};
pub use group::TypeGroup;
pub use impl_trait::TypeImplTrait;
pub use infer::TypeInfer;
pub use never::TypeNever;
pub use parentheses::TypeParentheses;
pub use path::TypePath;
pub use pointer::TypePointer;
pub use q_self::QSelf;
pub use r#macro::TypeMacro;
pub use r#return::ReturnType;
pub use reference::TypeReference;
pub use slice::TypeSlice;
pub use trait_object::TypeTraitObject;
pub use tuple::TypeTuple;

#[derive(Clone)]
pub enum Type {
    Array(TypeArray),
    BareFunction(TypeBareFunction),
    Group(TypeGroup),
    ImplTrait(TypeImplTrait),
    Infer(TypeInfer),
    Macro(TypeMacro),
    Never(TypeNever),
    Parentheses(TypeParentheses),
    Path(TypePath),
    Pointer(TypePointer),
    Reference(TypeReference),
    Slice(TypeSlice),
    TraitObject(TypeTraitObject),
    Tuple(TypeTuple),
    Verbatim(TokenStream),
}
