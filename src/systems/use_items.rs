use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesManaRestore)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
pub fn use_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(activation_message, activate)| {
            let item_to_activate = ecs.entry_ref(activate.item);
            if let Ok(item) = item_to_activate {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.user, healing.amount));
                }

                if let Ok(reveal) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }
            commands.remove(activate.item);
            commands.remove(*activation_message);
        });


    for heal in &healing_to_apply {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}