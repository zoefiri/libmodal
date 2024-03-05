use std::fmt::Debug;

#[derive(Debug, PartialEq, Hash)]
pub enum Input<K: Debug + PartialEq> {
    Single(K),
    Group(Vec<K>),
}
