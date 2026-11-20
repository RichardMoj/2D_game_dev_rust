use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(&WantsToAttack, Entity)>::query();

    let defenders: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(attack, entity)| (*entity, attack.victim))
        .collect();
}
