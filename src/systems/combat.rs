use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(HealthPool)]
#[read_component(NameLabel)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(
    ecs: &mut SubWorld, // mut coz we have potential to remove entities
    commands: &mut CommandBuffer,
) {
    // get a list of all the disputes that are planned
    let mut disputes = <(Entity, &WantsToAttack)>::query();
    // lets make a list of victims
    let victims: Vec<(Entity, Entity, Entity)> = disputes // REMEMBER, the first entity is the Message, NOT the attacker
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target, attack.attacker))
        .collect();

    victims.iter().for_each(|(message, victim, attacker)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap().get_component::<Player>().is_ok();
        let name = match ecs
            .entry_ref(*victim)
            .unwrap().get_component::<NameLabel>() {
            Ok(n) => n.0.clone(),
            Err(_) => "Unknown entity".to_string()
        };

        let base_damage = if let Ok(ent_ref) = ecs.entry_ref(*attacker) {
            if let Ok(d) = ent_ref.get_component::<Damage>() {
                println!("Base damage: {}", d.0);
                d.0
            } else {
                println!("No base damage");
                0
            }
        } else {
            0
        };

        let weapon_damage: u32 = <(&Carried, &Damage)>::query().iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, damage)| damage.0)
            .sum();

        //TODO: Modifiers

        //TODO: Damage amps and other affections (buffs?)
        let attacker_damage = (base_damage + weapon_damage) as i32;

        if let Ok(health_component) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<HealthPool>() {
            println!("{} health before attack: {}. Attack damage: {}", name, health_component.current, attacker_damage);

            // To make player not take damage
            //if !is_player {
            //    health_component.current -= 1;
            //}
            health_component.current -= attacker_damage; //TODO: Determine hit amount based on gear

            if health_component.current < 1 && !is_player {
                //TODO: death system
                //TODO: XP gain
                println!("{} dies!", name);
                commands.remove(*victim);
            }
        }
        // remove the attack command
        commands.remove(*message);
    });
}