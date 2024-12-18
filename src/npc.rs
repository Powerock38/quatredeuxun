use bevy::{color::palettes::css::RED, prelude::*};
use rand::prelude::*;

use crate::{
    dice::{Dice, InHandBundle, NewDiceCommand, RollDice, NB_DICES},
    player::PlayerDice,
    table::TRAY_RADIUS,
};

pub const NPC_POSITION: Vec3 = Vec3::new(0.0, TRAY_RADIUS * 1.5, -TRAY_RADIUS * 1.5);

#[derive(Event)]
pub struct NPCThrow;

pub fn spawn_npc_dices(mut commands: Commands) {
    for i in 0..NB_DICES {
        let entity = commands.spawn_empty().id();

        commands.add(NewDiceCommand {
            entity,
            i,
            tint_color: RED.into(),
        });

        commands
            .entity(entity)
            .observe(on_npc_throw)
            .insert(Transform::from_xyz(
                1000.0 + i as f32 * 100.0,
                1000.0,
                1000.0,
            ));
    }
}

pub fn roll_npc_dices(
    mut commands: Commands,
    mut q_dices: Query<Entity, (With<Dice>, Without<PlayerDice>)>,
) {
    for entity in &mut q_dices {
        commands.trigger_targets(NPCThrow, entity);
    }
}

pub fn on_npc_throw(
    trigger: Trigger<NPCThrow>,
    mut commands: Commands,
    mut q_dices: Query<(&Dice, &mut Transform), Without<PlayerDice>>,
) {
    let entity = trigger.entity();
    let (dice, mut transform) = q_dices.get_mut(entity).unwrap();

    let mut rng = thread_rng();

    commands.entity(entity).insert(InHandBundle::default());
    *transform = dice.in_hand_transform(NPC_POSITION);

    commands.trigger_targets(
        RollDice(Vec3::new(
            rng.gen_range(-TRAY_RADIUS..=TRAY_RADIUS),
            0.0,
            rng.gen_range(-TRAY_RADIUS..=TRAY_RADIUS),
        )),
        entity,
    );
}

pub fn reroll_fallen_npc_dices(
    mut commands: Commands,
    mut q_dices: Query<(Entity, &Transform), (With<Dice>, Without<PlayerDice>)>,
) {
    for (entity, transform) in &mut q_dices {
        if transform.translation.y < 0.0 {
            commands.trigger_targets(NPCThrow, entity);
        }
    }
}
