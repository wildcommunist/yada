use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use serde::Deserialize;
use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enemy {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollectableResource {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collider {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Wanderer {} //TODO: Limit wandering by setting home point and checking distance from it

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub point: Point,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mana {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XP {
    pub current: i32,
    pub max: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    CopperOre,
    IronOre,
    MithrilOre,
    Unknown
}

impl From<&String> for ResourceType {
    fn from(value: &String) -> Self {
        match value.to_lowercase().as_str() {
            "copperore" => ResourceType::CopperOre,
            "ironore" => ResourceType::IronOre,
            "mithrilore" => ResourceType::MithrilOre,
            _ => ResourceType::Unknown
        }
    }
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Resource {
    pub resource: ResourceType,
    pub amount: u8,
}

#[derive(Clone, PartialEq)]
pub struct NameLabel(pub String);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToPerformAction {
    pub issuer: Entity,
    pub action: Action,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    Attack,
    Gather,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToGather {
    pub source: Entity,
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProvidesManaRestore {
    pub amount: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Copy, Clone, Deserialize, Debug, PartialEq)]
pub enum ItemRarity {
    Poor,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Artifact,
    Unknown,
}

impl From<ItemRarity> for ColorPair {
    fn from(value: ItemRarity) -> Self {
        match value {
            ItemRarity::Poor => ColorPair::new((157, 157, 157), BLACK),
            ItemRarity::Common => ColorPair::new((255, 255, 255), BLACK),
            ItemRarity::Uncommon => ColorPair::new((30, 255, 0), BLACK),
            ItemRarity::Rare => ColorPair::new((0, 121, 221), BLACK),
            ItemRarity::Epic => ColorPair::new((163, 53, 238), BLACK),
            ItemRarity::Legendary => ColorPair::new((255, 128, 0), BLACK),
            ItemRarity::Artifact => ColorPair::new((230, 204, 128), BLACK),
            ItemRarity::Unknown => ColorPair::new((0, 204, 255), BLACK),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ActivateItem {
    pub user: Entity,
    pub item: Entity,
}