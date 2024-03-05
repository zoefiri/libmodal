use super::mode::Mode;
use super::resource::{Map, ResourceMap};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// Stores `Mode`s
///
/// # Generic Arguments
///
/// * `T` -  the type of the user-defined mode enum, the variants of this enum represent modes.
/// * `K` -  the type used as the key for the bind map contained in the modes, usually this is
/// probably just keycodes/uint/strings/etc.
///
pub struct ModeMap<T: Hash + Eq + Copy + Debug, K: Debug + Hash + PartialEq + Clone> {
    pub map: HashMap<T, Mode<T, K>>,
}

pub fn init_mode_map<T: Hash + Eq + Copy + Debug, K: Debug + Hash + PartialEq + Clone>(
    modes: Vec<Mode<T, K>>,
) -> ModeMap<T, K> {
    let mut mode_map = ModeMap {
        map: HashMap::new(),
    };

    // insert modes
    for mode in modes {
        let mode_id = mode.mode;

        // panic if attempting to insert two modes under the same key
        if let Some(existing_mode) = mode_map.map.insert(mode_id, mode) {
            panic!("mode map was initialized with mode {:?} and mode {:?} under the same mode identifier", mode_id, existing_mode.mode)
        }
    }

    mode_map
}

/// take a resource map and a mode->(bind -> handler) map, and run the handler with the given
/// resources
pub(crate) fn proc_bind<
    T: 'static + Hash + Eq + Copy + Debug,
    K: Debug + Hash + PartialEq + Clone,
>(
    resources: ResourceMap,
    modes: ModeMap<T, K>,
    bind: K,
) {
    // retrieve the current mode
    let mode = resources
        .get_resource::<T>()
        .as_ref()
        .downcast_ref::<T>()
        .unwrap()
        .clone();

    // run the active mode's handler for `bind`
    (modes.map[&mode].bind_map.get(&bind).unwrap().handler)(resources);
}
