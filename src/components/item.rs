use crate::prelude::*;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmuletOfYala;

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