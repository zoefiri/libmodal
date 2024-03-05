// this is just kind of a scratchpad file, atm.

mod bind_handler;
mod input;
mod mode;
mod mode_map;
mod resource;
use resource::{Map, ResourceMap};
use ropey::Rope;
use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Modes {
    Insert,
    Normal,
}

pub fn ins(resources: ResourceMap) {}
pub fn norm(resources: ResourceMap) {}

fn main() {
    mode_map::init_mode_map(vec![
        mode::init_mode(
            Modes::Insert,
            vec![
                bind_handler::new_handler(input::Input::Single('i'), ins),
                bind_handler::new_handler(input::Input::Single('n'), norm),
            ],
        ),
        // mode::init_mode(Modes::Normal, vec![]),
    ]);
}
