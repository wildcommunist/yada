use crate::prelude::*;

#[system]
#[read_component(WantsToGather)]
#[write_component(Resource)]
pub fn gather(
    ecs: &mut SubWorld, // mut coz we have potential to remove entities
    commands: &mut CommandBuffer,
) {
    // get a list of all the disputes that are planned
    let mut gathering_query = <(Entity, &WantsToGather)>::query();
    // lets make a list of victims
    let gathering_messages: Vec<(Entity, Entity, Entity)> = gathering_query // Remember, the first entity is the MESSAGE, NOT the gatherer
        .iter(ecs)
        .map(|(msg_entity, gather_request)| {
            (*msg_entity, gather_request.target, gather_request.source)
        })
        .collect();

    gathering_messages.iter().for_each(|(message, resource_entity, gatherer_entity)| {
        if let Ok(resource_component) = ecs
            .entry_mut(*resource_entity)
            .unwrap()
            .get_component_mut::<Resource>() {
            println!(
                "Resource {} before gather: {}",
                resource_component.resource,
                resource_component.amount
            );


            resource_component.amount -= 1;

            commands.push(
                (
                    Item,
                    ItemRarity::from(resource_component.resource),
                    NameLabel(resource_component.resource.into()),
                    Carried(*gatherer_entity)
                )
            );
            if resource_component.amount < 1 {
                // we exhausted the node of its minerals
                commands.remove(*resource_entity);
            }
        }
        // remove the gather command
        commands.remove(*message);
    });
}