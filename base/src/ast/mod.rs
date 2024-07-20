//! Representations of parts of Rust's AST to assist parsing and generating [`TokenStream`]s

// rustdoc imports
#[allow(unused_imports)]
use proc_macro::TokenStream;

mod declarations;
mod expressions;
mod generics;
mod lifetime;
mod path;
mod punctuated;
mod r#type;
mod variable_name;
mod visibility;
mod meta;

pub use meta::Meta;
pub use declarations::*;
pub use expressions::*;
pub use generics::*;
pub use lifetime::Lifetime;
pub use path::*;
pub use punctuated::Punctuated;
pub use r#type::Type;
pub use variable_name::VariableName;
pub use visibility::Visibility;
