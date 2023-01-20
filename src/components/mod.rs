mod item;
mod statistics;
mod resource;
mod render;
mod movement;
mod enemy;

pub use crate::components::item::*;
pub use crate::components::statistics::*;
pub use crate::components::resource::*;
pub use crate::components::render::*;
pub use crate::components::movement::*;
pub use crate::components::enemy::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}

#[derive(Clone, PartialEq)]
pub struct NameLabel(pub String);