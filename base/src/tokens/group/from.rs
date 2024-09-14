use crate::tokens::Group;

impl<'a> From<proc_macro::Group> for Group {
    fn from(group: proc_macro::Group) -> Self {
        Group::new_raw(group)
    }
}
