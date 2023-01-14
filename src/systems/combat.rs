use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld, // mut coz we have potential to remove entities
    commands: &mut CommandBuffer,
) {
    // get a list of all the disputes that are planned
    let mut disputes = <(Entity, &WantsToAttack)>::query();
    // lets make a list of victims
    let victims: Vec<(Entity, Entity)> = disputes // REMEMBER, the first entity is the Message, NOT the attacker
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect();

    victims.iter().for_each(|(message, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap().get_component::<Player>().is_ok();
        if let Ok(health_component) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>() {
            println!("Health before attack: {}", health_component.current);

            health_component.current -= 1;
            if health_component.current < 1 && !is_player{
                //TODO: death system
                commands.remove(*victim);
            }
        }
        // remove the attack command
        commands.remove(*message);
    });
}