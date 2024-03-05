use chashmap::CHashMap;
use super::input::Input;
use std::{cmp::Eq, hash::Hash, fmt::Debug};

//  # Control Flow
//  - your main event loop calls process_bind() on bindsym
//      - current mode map (or a fallback chain mode map) has bindsym: call bind handler w/
//      resource map
//      - bindsym not in current mode map or its fallbacks: call mode's default bind handler
//      with the bind as param
//
//  - a: bind handler returns None - do nothing. process following binds normally
//  - b: bind handler returns Some(Chain{})
//      - post_handler: PostHandler{handler: BindHandler, trigger:String} - handler is called
//      when a change occurs on resource @ `trigger`, useful for verb-noun modal bind chaining
//      a'la `dw` (delete to start of next word)

#[derive(Debug)]
struct BindHandler<K: Debug + Hash + PartialEq> {
    key: K,
    pub handler: fn(resources: super::resource::ResourceMap),
} 

#[derive(Debug)]
pub struct Mode<T: Hash + Eq + Copy + Debug, K: Debug + Hash + PartialEq> {
    pub mode: T,
    pub bind_map: CHashMap<K, BindHandler<K>>,
}

pub fn init_mode<T: Hash + Eq + Copy + Debug, K: Debug + Hash + PartialEq>(mode_id: T, handlers: Vec<BindHandler<Input<K>>>) -> Mode<T, K> {
    let mut mode = Mode{
        mode: mode_id,
        bind_map: CHashMap::new(),
    };

    // insert handlers
    for handler in handlers {
        let key = handler.key;

        // panic if attempting to insert two handlers under the same key
        match key {
            Input::Single(input) => {
                if let Some(existing_bind_handler) = mode.bind_map.insert(input, BindHandler{key: input, handler: handler.handler}) {
                    panic!("mode map was initialized with handler {:?} and handler {:?} under the same key", handler, existing_bind_handler)
                }
            }, 
            Input::Group(inputs) => {
                for input in inputs {
                    if let Some(existing_bind_handler) = mode.bind_map.insert(input, BindHandler{key: input, handler: handler.handler}) {
                        panic!("mode map was initialized with handler {:?} and handler {:?} under the same key", handler, existing_bind_handler)
                    }
                }
            }
        }
    }

    mode
}

// pub struct Chain {
//     resource_change_trigger: TypeId, 
// }
