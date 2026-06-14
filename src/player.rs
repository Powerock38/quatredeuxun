use avian3d::prelude::*;
use bevy::{
    camera_controller::free_camera::FreeCamera, color::palettes::css::BLUE,
    picking::pointer::PointerButton, prelude::*,
};

use crate::{
    dice::{Dice, InHand, InHandBundle, NB_DICES, NewDiceCommand, RollDice},
    game::RetriesLeft,
    table::TRAY_RADIUS,
};

pub const PLAYER_POSITION: Vec3 = Vec3::new(0.0, TRAY_RADIUS * 1.5, TRAY_RADIUS * 1.5);

#[derive(Component)]
pub struct PlayerDice;

#[derive(Resource)]
pub struct SelectedDice(pub Entity);

#[derive(EntityEvent)]
pub struct PickupDice {
    pub entity: Entity,
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(PLAYER_POSITION).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

pub fn toggle_free_camera(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_camera: Single<(Entity, Option<&FreeCamera>), With<Camera3d>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let (camera_entity, free_camera) = *q_camera;
        if free_camera.is_some() {
            commands
                .entity(camera_entity)
                .insert(
                    Transform::from_translation(PLAYER_POSITION).looking_at(Vec3::ZERO, Dir3::Y),
                )
                .remove::<FreeCamera>();
        } else {
            commands.entity(camera_entity).insert(FreeCamera::default());
        }
    }
}

pub fn spawn_player_dices(mut commands: Commands) {
    for i in 0..NB_DICES {
        let entity = commands.spawn_empty().id();

        commands.queue(NewDiceCommand {
            entity,
            i,
            tint_color: BLUE.into(),
        });

        commands
            .entity(entity)
            .insert(PlayerDice)
            .observe(on_pickup_dice)
            .observe(on_click_dice);

        commands.trigger(PickupDice { entity });
    }
}

pub fn on_pickup_dice(
    trigger: On<PickupDice>,
    mut commands: Commands,
    mut q_dices: Query<(&Dice, &mut Transform), With<PlayerDice>>,
) {
    let entity = trigger.entity;
    let (dice, mut transform) = q_dices.get_mut(entity).unwrap();

    commands.entity(entity).insert(InHandBundle::default());
    *transform = dice.in_hand_transform(PLAYER_POSITION);

    commands.insert_resource(SelectedDice(entity));
}

/// Handles both left-click (select in-hand dice) and right-click (pick up table dice).
fn on_click_dice(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    q_dices_in_hand: Query<(), (With<PlayerDice>, With<InHand>)>,
    q_dices_on_table: Query<(), (With<PlayerDice>, Without<InHand>)>,
    mut retries: ResMut<RetriesLeft>,
) {
    let entity = trigger.entity;
    let button = trigger.event().button;

    if button == PointerButton::Primary && q_dices_in_hand.get(entity).is_ok() {
        // Left-click an in-hand dice → select it
        commands.insert_resource(SelectedDice(entity));
    } else if button == PointerButton::Secondary
        && retries.0 > 0
        && q_dices_on_table.get(entity).is_ok()
    {
        // Right-click a table dice → pick it up (costs a retry)
        commands.trigger(PickupDice { entity });
        retries.0 -= 1;
    }
}

pub fn on_click_table(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    q_dices_in_hand: Query<Entity, (With<PlayerDice>, With<InHand>)>,
    selected_dice: Option<Res<SelectedDice>>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let Some(selected_entity) = selected_dice.as_ref().map(|s| s.0) else {
        return;
    };

    let hit_position = trigger.event().hit.position.unwrap_or_default();

    commands.trigger(RollDice {
        entity: selected_entity,
        target_position: hit_position,
    });

    // Auto-select the next dice in hand, or clear the selection
    if let Some(next) = q_dices_in_hand.iter().find(|&e| e != selected_entity) {
        commands.insert_resource(SelectedDice(next));
    } else {
        commands.remove_resource::<SelectedDice>();
    }
}

pub fn manage_selected_dice_animation(
    selected_dice: Option<Res<SelectedDice>>,
    mut existed: Local<bool>,
    mut q_dices_in_hand: Query<
        (&mut AngularVelocity, &mut Rotation),
        (With<PlayerDice>, With<InHand>),
    >,
) {
    // Closure to reset the dice rotation
    let mut reset_dice_states = || {
        for (mut angular_velocity, mut rotation) in &mut q_dices_in_hand {
            angular_velocity.0 = Vec3::ZERO;
            *rotation = Rotation::default();
        }
    };

    if let Some(selected_dice) = selected_dice {
        *existed = true;

        if selected_dice.is_added() || selected_dice.is_changed() {
            reset_dice_states();

            if let Ok((mut angular_velocity, _)) = q_dices_in_hand.get_mut(selected_dice.0) {
                angular_velocity.0 = Vec3::splat(0.5);
            }
        }
    } else if *existed {
        *existed = false;
        reset_dice_states();
    }
}

pub fn pickup_fallen_dices(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<PlayerDice>>,
) {
    for (entity, transform) in &query {
        if transform.translation.y < 0.0 {
            // Cassé ! Pick up the dice
            commands.trigger(PickupDice { entity });
        }
    }
}

pub fn pickup_all_player_dices(mut commands: Commands, query: Query<Entity, With<PlayerDice>>) {
    for entity in &query {
        commands.trigger(PickupDice { entity });
    }

    commands.insert_resource(RetriesLeft::default());
}
