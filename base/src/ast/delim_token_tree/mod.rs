use crate::tokens::Group;

mod deref;
mod from;
mod parse;
mod to_tokens;

/// A [`Group`] which may be either owned or borrowed
#[derive(Debug, Clone)]
pub enum DelimTokenTree<'a> {
    /// The group is borrowed
    Borrowed(&'a Group),

    /// The group is owned
    Owned(Group),
}
