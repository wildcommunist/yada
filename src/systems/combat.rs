use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(NameLabel)]
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
        let name = match ecs
            .entry_ref(*victim)
            .unwrap().get_component::<NameLabel>() {
            Ok(n) => n.0.clone(),
            Err(_) => "Unknown entity".to_string()
        };

        if let Ok(health_component) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>() {
            println!("Health before attack: {}", health_component.current);

            // To make player not take damage
            if !is_player {
                health_component.current -= 1;
            }
            //health_component.current -= 1; //TODO: Determine hit amount based on gear

            if health_component.current < 1 && !is_player {
                //TODO: death system
                println!("{} dies!", name);
                commands.remove(*victim);
            }
        }
        // remove the attack command
        commands.remove(*message);
    });
}