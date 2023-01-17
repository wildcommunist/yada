use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub name: String,
    pub provides: Option<Vec<(String, i32, Option<i32>)>>,
    pub icon: char,
    pub levels: HashSet<usize>,
    pub quality: Option<ItemRarity>,
    pub frequency: i32,
    pub hp: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
    Resource,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/templates.ron")
            .expect("Failed to open template file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self, ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        // USing a vec to hold references to templates automatically bakes in their frequency.
        // each template is stored the number of times as their frequency. Whenever we pick a random
        // template from a vec, the template added there as many times as the frequency
        let mut available_entities = Vec::new();
        self.entities.iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|e| {
                for _ in 0..e.frequency {
                    available_entities.push(e);
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|sp| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                // pick a random entity for the spawn point from the vec
                self.spawn_entity(sp, entity, &mut commands, rng);
            }
        });
        commands.flush(ecs);
    }

    fn spawn_entity(
        &self,
        position: &Point,
        template: &Template,
        commands: &mut CommandBuffer,
        rng: &mut RandomNumberGenerator,
    ) {
        let entity = commands.push(
            (
                *position,
                Render {
                    color: ColorPair::new(WHITE, BLACK),
                    glyph: to_cp437(template.icon),
                },
                NameLabel(template.name.clone())
            )
        );

        match template.entity_type {
            EntityType::Enemy => {
                if let Some(q) = template.quality {
                    commands.add_component(entity, q);
                }
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, Collider {});
                commands.add_component(entity, Wanderer {});
                commands.add_component(entity, ChasingPlayer {});
                commands.add_component(entity, FieldOfView::new(6));

                if let Some(hp) = template.hp {
                    commands.add_component(entity, Health { current: hp, max: hp });
                }
            }
            EntityType::Item => {
                commands.add_component(entity, Item);
                if let Some(q) = template.quality {
                    commands.add_component(entity, q);
                }

                if let Some(provider) = &template.provides {
                    provider.iter().for_each(|(provided, amount, _amount2)| {
                        match provided.to_lowercase().as_str() {
                            "mana" => {
                                commands.add_component(entity, ProvidesManaRestore { amount: *amount });
                            }
                            "health" => {
                                commands.add_component(entity, ProvidesHealing { amount: *amount });
                            }
                            "mapreveal" => {
                                commands.add_component(entity, ProvidesDungeonMap)
                            }
                            _ => {}
                        }
                    });
                }
            }
            EntityType::Resource => {
                if let Some(provider) = &template.provides {
                    provider.iter().for_each(|(item, range_start, range_end)| {
                        let resource = ResourceType::from(item);
                        let max_amount = range_end.unwrap_or(*range_start + 1);
                        commands.add_component(entity, Collider {});
                        commands.add_component(
                            entity, Resource {
                                resource,
                                amount: rng.range(*range_start, max_amount) as u8,
                            },
                        ); //TODO: resource types
                    });
                }
            }
        }
    }
}