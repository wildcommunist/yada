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
    let gathering_messages: Vec<(Entity, Entity)> = gathering_query // Remember, the first entity is the MESSAGE, NOT the gatherer
        .iter(ecs)
        .map(|(msg_entity, gather_request)| (*msg_entity, gather_request.target))
        .collect();

    gathering_messages.iter().for_each(|(message, gather_target)| {
        if let Ok(resource_component) = ecs
            .entry_mut(*gather_target)
            .unwrap()
            .get_component_mut::<Resource>() {
            println!("Resource {} before gather: {}", resource_component.resource.to_string(), resource_component.amount);

            //resource_component.amount -= 1;
            //if resource_component.amount < 1 {
            //    //TODO: add to player inventory system
            //    commands.remove(*gather_target);
            //}
        }
        // remove the attack command
        commands.remove(*message);
    });
}