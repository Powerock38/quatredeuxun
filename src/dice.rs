use avian3d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use crate::combination::{Combination, DiceResult, LastCombination};
use crate::game::GameState;
use crate::player::PlayerDice;
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
    pub size: f32,
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

    pub fn in_hand_transform(&self, thrower_position: Vec3) -> Transform {
        Transform::from_translation(
            thrower_position
                + Vec3::new(
                    (self.i as f32 - NB_DICES as f32 / 2.0 + 0.5) * self.size,
                    -6.0,
                    -0.2,
                ),
        )
    }
}

#[derive(Event)]
pub struct RollDice(pub Vec3);

#[derive(Component)]
pub struct InHand;

#[derive(Bundle)]
pub struct InHandBundle {
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

pub fn new_dice(commands: &mut Commands, assets_server: &Res<AssetServer>, i: usize) -> Entity {
    let dice = Dice::new_6(i);

    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(dice.size, dice.size, dice.size),
            LinearDamping(0.5),
            AngularDamping(0.5),
            ColliderDensity(5.0),
            SceneBundle {
                scene: assets_server.load(GltfAssetLabel::Scene(0).from_asset("dice.glb")),
                ..default()
            },
            dice,
            InHandBundle::default(),
        ))
        .observe(on_roll_dice)
        .id()
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

pub fn analyze_dices(
    mut commands: Commands,
    last_combination: Option<Res<LastCombination>>,
    collisions: Res<Collisions>,
    q_table_parts: Query<Entity, With<TablePart>>,
    q_player_dices_on_table: Query<
        (Entity, &Dice, &Transform, &AngularVelocity, &LinearVelocity),
        (Without<InHand>, With<PlayerDice>),
    >,
    q_npc_dices_on_table: Query<
        (Entity, &Dice, &Transform, &AngularVelocity, &LinearVelocity),
        (Without<InHand>, Without<PlayerDice>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // FIXME: deadlock between LastCombination / GameState
    if last_combination.is_none() {
        let read_dice = |args: (Entity,
                          &Dice,
                          &Transform,
                          &AngularVelocity,
                          &LinearVelocity)|
         -> Option<DiceResult> {
            let (entity, dice, transform, angular_velocity, linear_velocity) = args;

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

                return Some(result as DiceResult);
            }

            None
        };

        let results_npc = q_npc_dices_on_table
            .iter()
            .filter_map(read_dice)
            .collect::<Vec<_>>();

        if results_npc.len() == NB_DICES {
            next_state.set(GameState::PlayerRolling);
        }

        let results_player = q_player_dices_on_table
            .iter()
            .filter_map(read_dice)
            .collect::<Vec<_>>();

        if results_player.len() == NB_DICES && results_npc.len() == NB_DICES {
            commands.insert_resource(LastCombination {
                player: Combination::get(results_player),
                npc: Combination::get(results_npc),
            });

            next_state.set(GameState::NPCRolling);
        }
    }
}
