use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod player;
mod killzone;
mod enemy;
mod spikes;
mod invisible_tile;
mod moving_trap;
mod timer_trap;
mod coin;
mod tree_trap;
