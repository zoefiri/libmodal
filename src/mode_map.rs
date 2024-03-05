use super::mode::Mode;
use std::{collections::HashMap, hash::Hash, fmt::Debug};

/// Stores `Mode`s
///
/// # Generic Arguments
///
/// * `T` -  the type of the user-defined mode enum, the variants of this enum represent modes.
/// * `K` -  the type used as the key for the bind map contained in the modes, usually this is
/// probably just keycodes/uint/strings/etc.
///
pub struct ModeMap<T: Hash + Eq + Copy + Debug, K: Debug + Hash + PartialEq> {
    pub map: HashMap<T, Mode<T, K>>,
}

pub fn init_mode_map<T: Hash + Eq + Copy + Debug, K: Debug + Hash + PartialEq>(modes: Vec<Mode<T, K>>) -> ModeMap<T, K> {
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
