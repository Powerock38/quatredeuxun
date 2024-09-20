use avian3d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use crate::combination::{Combination, DiceResult, LastCombination};
use crate::game::{ToBeat, Tries};
use crate::player::PLAYER_POSITION;
use crate::table::TablePart;

pub const NB_DICES: usize = 3;
pub const MIN_NB_DICES: usize = 2;
const MAX_ANGULAR_SPEED: f32 = 10.0;
const MIN_FORCE: f32 = 50.0;
const MAX_FORCE: f32 = 100.0;
const MIN_MOVEMENT: f32 = 0.3;

#[derive(Component)]
pub struct Dice {
    i: usize,
    size: f32,
    face_normals: Vec<Vec3>,
}

impl Dice {
    pub fn new_6(i: usize) -> Self {
        Self {
            i,
            size: 1.0,
            face_normals: vec![
                Vec3::new(0.0, 1.0, 0.0),  // Top face
                Vec3::new(1.0, 0.0, 0.0),  // Right face
                Vec3::new(0.0, 0.0, -1.0), // Back face
                Vec3::new(0.0, 0.0, 1.0),  // Front face
                Vec3::new(-1.0, 0.0, 0.0), // Left face
                Vec3::new(0.0, -1.0, 0.0), // Bottom face
            ],
        }
    }

    pub fn in_hand_transform(&self) -> Transform {
        Transform::from_translation(
            PLAYER_POSITION
                + Vec3::new(
                    (self.i as f32 - NB_DICES as f32 / 2.0 + 0.5) * self.size,
                    -6.0,
                    -0.2,
                ),
        )
    }
}

#[derive(Resource)]
pub struct SelectedDice(pub Entity);

#[derive(Event)]
pub struct RollDice(pub Vec3);

#[derive(Event)]
pub struct PickupDice;

#[derive(Component)]
pub struct InHand;

#[derive(Bundle)]
struct InHandBundle {
    in_hand: InHand,
    locked_axes: LockedAxes,
}

impl Default for InHandBundle {
    fn default() -> Self {
        Self {
            in_hand: InHand,
            locked_axes: LockedAxes::TRANSLATION_LOCKED,
        }
    }
}

pub fn spawn_dices(mut commands: Commands, assets_server: Res<AssetServer>) {
    for i in 0..NB_DICES {
        let dice = Dice::new_6(i);

        let entity = commands
            .spawn((
                RigidBody::Dynamic,
                Collider::cuboid(dice.size, dice.size, dice.size),
                LinearDamping(0.3),
                SceneBundle {
                    scene: assets_server.load(GltfAssetLabel::Scene(0).from_asset("dice.glb")),
                    ..default()
                },
                dice,
            ))
            .observe(on_roll_dice)
            .observe(on_pickup_dice)
            .id();

        commands.trigger_targets(PickupDice, entity);
    }
}

pub fn on_roll_dice(
    trigger: Trigger<RollDice>,
    mut commands: Commands,
    mut q_dices: Query<(&Transform, &mut AngularVelocity, &mut LinearVelocity), With<Dice>>,
    time: Res<Time>,
) {
    let entity = trigger.entity();
    let (transform, mut angular_velocity, mut linear_velocity) = q_dices.get_mut(entity).unwrap();

    // Release the dice from the hand
    commands.remove_resource::<LastCombination>();
    commands.entity(entity).remove::<InHandBundle>();

    // Roll the dice
    let mut rng = thread_rng();

    let trajectory = trigger.event().0 - transform.translation;
    let force = trajectory * rng.gen_range(MIN_FORCE..MAX_FORCE);
    linear_velocity.0 = force * time.delta_seconds();

    angular_velocity.0 = Vec3::new(
        rng.gen_range(-MAX_ANGULAR_SPEED..MAX_ANGULAR_SPEED),
        rng.gen_range(-MAX_ANGULAR_SPEED..MAX_ANGULAR_SPEED),
        rng.gen_range(-MAX_ANGULAR_SPEED..MAX_ANGULAR_SPEED),
    );
}

pub fn on_pickup_dice(
    trigger: Trigger<PickupDice>,
    mut commands: Commands,
    mut q_dices: Query<(&Dice, &mut Transform)>,
) {
    let entity = trigger.entity();
    let (dice, mut transform) = q_dices.get_mut(entity).unwrap();

    commands.entity(entity).insert(InHandBundle::default());
    *transform = dice.in_hand_transform();

    commands.insert_resource(SelectedDice(entity));
}

pub fn analyze_dices(
    mut commands: Commands,
    last_combination: Option<Res<LastCombination>>,
    mut tries: ResMut<Tries>,
    to_beat: Res<ToBeat>,
    collisions: Res<Collisions>,
    q_table_parts: Query<Entity, With<TablePart>>,
    q_dices_on_table: Query<
        (Entity, &Dice, &Transform, &AngularVelocity, &LinearVelocity),
        Without<InHand>,
    >,
    q_dices_in_hand: Query<(), (With<Dice>, With<InHand>)>,
) {
    if last_combination.is_none() {
        let nb_dices_total = q_dices_on_table.iter().count() + q_dices_in_hand.iter().count();
        let mut results = vec![];

        for (entity, dice, transform, angular_velocity, linear_velocity) in &q_dices_on_table {
            if linear_velocity.0.length() < MIN_MOVEMENT
                && angular_velocity.0.length() < MIN_MOVEMENT
                && q_table_parts
                    .iter()
                    .any(|table_part| collisions.contains(entity, table_part))
            {
                // Determine which face is most aligned with the world up vector
                let mut max_dot = -1.0;
                let mut result = 0;

                for (i, normal) in dice.face_normals.iter().enumerate() {
                    // Rotate the face normal to the current orientation
                    let transformed_normal = transform.rotation * *normal;

                    // Compare it with the world up vector
                    let dot_product = transformed_normal.dot(Vec3::Y);

                    // Check if this face is more aligned with the up direction
                    if dot_product > max_dot {
                        max_dot = dot_product;
                        result = i + 1;
                    }
                }

                results.push(result as DiceResult);
            }
        }

        if results.len() == nb_dices_total {
            let combination = Combination::get(results);

            // Update the player's tries
            tries.0 += 1;

            let win = to_beat.is_won(&combination);

            if win || tries.0 >= to_beat.tries {
                commands.insert_resource(ToBeat::roll());
                tries.0 = 0;
            }

            commands.insert_resource(LastCombination { combination, win });
        }
    }
}

pub fn pickup_fallen_dices(mut commands: Commands, query: Query<(Entity, &Transform), With<Dice>>) {
    for (entity, transform) in &query {
        if transform.translation.y < 0.0 {
            // Cassé ! Pick up the dice
            commands.trigger_targets(PickupDice, entity);
        }
    }
}

pub fn filter_collisions_in_hand(
    mut collisions: ResMut<Collisions>,
    query: Query<(), (With<Dice>, With<InHand>)>,
) {
    collisions
        .retain(|contacts| !query.contains(contacts.entity1) && !query.contains(contacts.entity2));
}

pub fn raycast_dices(
    mut commands: Commands,
    q_rays: Query<(Entity, &RayCaster, &RayHits)>,
    q_dices_in_hand: Query<Entity, (With<Dice>, With<InHand>)>,
    q_dices_on_table: Query<(), (With<Dice>, Without<InHand>)>,
    q_table: Query<(), With<TablePart>>,
    selected_dice: Option<Res<SelectedDice>>,
    last_combination: Option<Res<LastCombination>>,
) {
    for (ray_entity, ray, hits) in &q_rays {
        for hit in hits.iter_sorted() {
            // Pick up the dices on the table
            if q_dices_on_table.get(hit.entity).is_ok() && last_combination.is_some() {
                commands.trigger_targets(PickupDice, hit.entity);
                break;
            }

            // Select dices in hand
            if q_dices_in_hand.get(hit.entity).is_ok() {
                commands.insert_resource(SelectedDice(hit.entity));
                break;
            }

            // Click table to roll the dices
            if q_table.get(hit.entity).is_ok() {
                if let Some(entity) = selected_dice.as_ref().map(|selected_dice| selected_dice.0) {
                    let point = ray.origin + *ray.direction * hit.time_of_impact;

                    commands.trigger_targets(RollDice(point), entity);

                    if let Some(entity) = q_dices_in_hand.iter().find(|e| *e != entity) {
                        commands.insert_resource(SelectedDice(entity));
                    } else {
                        commands.remove_resource::<SelectedDice>();
                    }

                    break;
                }
            }
        }

        commands.entity(ray_entity).despawn();
    }
}

pub fn manage_selected_dice_animation(
    selected_dice: Option<Res<SelectedDice>>,
    mut existed: Local<bool>,
    mut q_dices_in_hand: Query<(&mut AngularVelocity, &mut Rotation), (With<Dice>, With<InHand>)>,
) {
    // Closure to reset the dice states
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
