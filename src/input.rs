use std::fmt::Debug;

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum Input<K: Debug + PartialEq> {
    Single(K),
    Group(Vec<K>),
}
