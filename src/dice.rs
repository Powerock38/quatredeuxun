use avian3d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use crate::combination::{Combination, DiceResult};
use crate::table::{RING_HEIGHT, TABLE_RADIUS};
use crate::ui::LastCombination;

pub const NB_DICES: usize = 3;
const DICE_SIZE: f32 = 1.0;
const MAX_ANGULAR_SPEED: f32 = 10.0;
const MIN_FORCE: f32 = 500.0;
const MAX_FORCE: f32 = 1000.0;
const ROLL_START_ANGLE: f32 = std::f32::consts::FRAC_PI_4;
const CONE_ANGLE: f32 = std::f32::consts::FRAC_PI_4;
const MIN_MOVEMENT: f32 = 0.3;

fn roll_start(i: usize) -> Vec3 {
    let roll_start_angle = ROLL_START_ANGLE + i as f32 * std::f32::consts::FRAC_PI_8;
    Vec3::new(
        roll_start_angle.cos() * TABLE_RADIUS,
        RING_HEIGHT * 2.0,
        roll_start_angle.sin() * TABLE_RADIUS,
    )
}

#[derive(Component)]
pub struct Dice {
    i: usize,
}

impl Dice {
    pub fn face_normals(&self) -> [Vec3; 6] {
        [
            Vec3::new(0.0, 1.0, 0.0),  // Top face
            Vec3::new(1.0, 0.0, 0.0),  // Right face
            Vec3::new(0.0, 0.0, -1.0), // Back face
            Vec3::new(0.0, 0.0, 1.0),  // Front face
            Vec3::new(-1.0, 0.0, 0.0), // Left face
            Vec3::new(0.0, -1.0, 0.0), // Bottom face
        ]
    }
}

pub fn spawn_dices(mut commands: Commands, assets_server: Res<AssetServer>) {
    for i in 0..NB_DICES {
        commands.spawn((
            Dice { i },
            RigidBody::Dynamic,
            Collider::cuboid(DICE_SIZE, DICE_SIZE, DICE_SIZE),
            LinearDamping(0.3),
            AngularDamping(0.3),
            SceneBundle {
                scene: assets_server.load(GltfAssetLabel::Scene(0).from_asset("dice.glb")),
                transform: Transform::from_xyz(
                    i as f32 * 3.0 * if i % 2 == 0 { 1.0 } else { -1.0 },
                    4.0,
                    0.0,
                ),
                ..default()
            },
        ));
    }
}

pub fn roll_dices(
    mut commands: Commands,
    last_combination: Option<Res<LastCombination>>,
    button_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &Dice,
        &mut Transform,
        &mut AngularVelocity,
        &mut LinearVelocity,
    )>,
    time: Res<Time>,
) {
    let mut rng = thread_rng();

    if button_input.just_pressed(KeyCode::KeyR) {
        commands.remove_resource::<LastCombination>();

        for (dice, mut transform, mut angular_velocity, mut linear_velocity) in &mut query {
            let roll_start = roll_start(dice.i);

            let origin = Vec3::ZERO;
            let forward_direction = (origin - roll_start).normalize();

            transform.translation = roll_start;

            // Generate a random angle within the cone
            let random_angle = rng.gen_range(0.0..CONE_ANGLE);
            let random_azimuth = rng.gen_range(0.0..std::f32::consts::TAU);

            // Create a random direction vector within the cone
            let cone_direction = Vec3::new(
                random_angle.sin() * random_azimuth.cos(),
                random_angle.sin() * random_azimuth.sin(),
                random_angle.cos(),
            );

            // Align the cone direction with the forward direction
            let roll_direction =
                Quat::from_rotation_arc(Vec3::Z, forward_direction) * cone_direction;

            let force = roll_direction * rng.gen_range(MIN_FORCE..MAX_FORCE);
            linear_velocity.0 = force * time.delta_seconds();

            angular_velocity.0 = Vec3::new(
                rng.gen_range(-MAX_ANGULAR_SPEED..MAX_ANGULAR_SPEED),
                rng.gen_range(-MAX_ANGULAR_SPEED..MAX_ANGULAR_SPEED),
                rng.gen_range(-MAX_ANGULAR_SPEED..MAX_ANGULAR_SPEED),
            );
        }
    }

    if last_combination.is_none() {
        let results: &mut [u8; NB_DICES] = &mut [0; NB_DICES];
        let mut nb_results = 0;

        for (dice, transform, mut angular_velocity, mut linear_velocity) in &mut query {
            if linear_velocity.0.length() < MIN_MOVEMENT
                && angular_velocity.0.length() < MIN_MOVEMENT
            {
                linear_velocity.0 = Vec3::ZERO;
                angular_velocity.0 = Vec3::ZERO;

                // Determine which face is most aligned with the world up vector
                let mut max_dot = -1.0;
                let mut result = 0;

                for (i, normal) in dice.face_normals().iter().enumerate() {
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

                results[dice.i] = result as DiceResult;
                nb_results += 1;
            }
        }

        if nb_results == NB_DICES {
            let combination = Combination::get(results);
            commands.insert_resource(LastCombination(combination));
        }
    }
}

pub fn teleport_fallen_dices(mut query: Query<(&Dice, &mut Transform)>) {
    for (dice, mut transform) in &mut query {
        if transform.translation.y < 0.0 {
            transform.translation = roll_start(dice.i);
        }
    }
}
