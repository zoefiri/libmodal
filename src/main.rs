mod input;
mod mode;
mod mode_map;
mod resource;
use std::{hash::Hash, fmt::Debug};
use resource::{ResourceMap, Map};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Modes {
    Insert,
    Normal,
}

/// take a resource map and a mode->(bind -> handler) map, and run the handler with the given
/// resources
fn proc_bind<T: Hash + Eq + Copy + Debug, K: Debug + Hash + PartialEq>(resources: ResourceMap, modes: mode_map::ModeMap<T, K>, bind: K) {
    // retrieve the current mode
    let mode: T;
    {
        mode = resources.get_resource::<T>().as_ref().downcast_ref::<T>().unwrap().clone();
    }

    // run the active mode's handler for `bind`
    (modes.map[&mode].bind_map.get(&bind).unwrap().handler)(resources);
}

fn main() {
    mode_map::init_mode_map(vec![
        mode::init_mode(Modes::Insert, vec![]),
        mode::init_mode(Modes::Normal, vec![]),
    ]);
}
