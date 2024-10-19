use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    dice::{new_dice, Dice, InHand, InHandBundle, RollDice, NB_DICES},
    game::ThrowsLeft,
    table::{TablePart, TABLE_RADIUS},
};

pub const PLAYER_POSITION: Vec3 = Vec3::new(0.0, TABLE_RADIUS * 1.5, TABLE_RADIUS * 1.5);

#[derive(Component)]
pub struct PlayerDice;

#[derive(Resource)]
pub struct SelectedDice(pub Entity);

#[derive(Event)]
pub struct PickupDice;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        crate::flycam::FlyCam,
        Camera3dBundle {
            transform: Transform::from_translation(PLAYER_POSITION).looking_at(Vec3::ZERO, Dir3::Y),
            ..default()
        },
    ));
}

pub fn spawn_player_dices(mut commands: Commands, assets_server: Res<AssetServer>) {
    for i in 0..NB_DICES {
        let entity = new_dice(&mut commands, &assets_server, i);

        commands
            .entity(entity)
            .insert(PlayerDice)
            .observe(on_pickup_dice);

        commands.trigger_targets(PickupDice, entity);
    }
}

pub fn on_pickup_dice(
    trigger: Trigger<PickupDice>,
    mut commands: Commands,
    mut q_dices: Query<(&Dice, &mut Transform), With<PlayerDice>>,
) {
    let entity = trigger.entity();
    let (dice, mut transform) = q_dices.get_mut(entity).unwrap();

    commands.entity(entity).insert(InHandBundle::default());
    *transform = dice.in_hand_transform(PLAYER_POSITION);

    commands.insert_resource(SelectedDice(entity));
}

#[derive(Component)]
pub enum ClickType {
    Left,
    Right,
}

pub fn click_spawns_raycast(
    mut commands: Commands,
    button_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let cursor_position = if button_input.just_pressed(MouseButton::Left)
        || button_input.just_pressed(MouseButton::Right)
    {
        windows.single().cursor_position()
    } else {
        touches
            .iter_just_pressed()
            .next()
            .map(|touch| touch.position())
    };

    let Some(cursor_position) = cursor_position else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    commands.spawn((
        RayCaster::from_ray(ray),
        if button_input.just_pressed(MouseButton::Right) {
            ClickType::Right
        } else {
            ClickType::Left
        },
    ));
}

pub fn raycast_dices(
    mut commands: Commands,
    q_rays: Query<(Entity, &RayCaster, &RayHits, &ClickType)>,
    q_dices_in_hand: Query<Entity, (With<PlayerDice>, With<InHand>)>,
    q_dices_on_table: Query<(), (With<PlayerDice>, Without<InHand>)>,
    q_table: Query<(), With<TablePart>>,
    selected_dice: Option<Res<SelectedDice>>,
    mut throws: ResMut<ThrowsLeft>,
) {
    for (ray_entity, ray, hits, click_type) in &q_rays {
        for hit in hits.iter_sorted() {
            // Select dices in hand
            if q_dices_in_hand.get(hit.entity).is_ok() {
                commands.insert_resource(SelectedDice(hit.entity));
                break;
            }

            if throws.0 > 0 {
                if matches!(click_type, ClickType::Right) {
                    // Pick up the dices on the table
                    if q_dices_on_table.get(hit.entity).is_ok() {
                        commands.trigger_targets(PickupDice, hit.entity);
                        break;
                    }
                }

                // Click table to roll the dices
                if q_table.get(hit.entity).is_ok() {
                    if let Some(entity) =
                        selected_dice.as_ref().map(|selected_dice| selected_dice.0)
                    {
                        let point = ray.origin + *ray.direction * hit.time_of_impact;

                        commands.trigger_targets(RollDice(point), entity);

                        throws.0 -= 1;

                        if let Some(entity) = q_dices_in_hand.iter().find(|e| *e != entity) {
                            commands.insert_resource(SelectedDice(entity));
                        } else {
                            commands.remove_resource::<SelectedDice>();
                        }

                        break;
                    }
                }
            }
        }

        commands.entity(ray_entity).despawn();
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
    mut throws: ResMut<ThrowsLeft>,
) {
    for (entity, transform) in &query {
        if transform.translation.y < 0.0 {
            // Cassé ! Pick up the dice
            commands.trigger_targets(PickupDice, entity);
            throws.0 += 1;
        }
    }
}

pub fn filter_collisions_in_hand(
    mut collisions: ResMut<Collisions>,
    query: Query<(), (With<PlayerDice>, With<InHand>)>,
) {
    collisions
        .retain(|contacts| !query.contains(contacts.entity1) && !query.contains(contacts.entity2));
}

pub fn pickup_all_player_dices(mut commands: Commands, query: Query<Entity, With<PlayerDice>>) {
    for entity in &query {
        commands.trigger_targets(PickupDice, entity);
    }

    commands.insert_resource(ThrowsLeft::default());
}
