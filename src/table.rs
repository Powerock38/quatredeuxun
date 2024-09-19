use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};

use crate::{
    combination::LastCombination,
    dice::{Dice, InHand},
};

pub const TABLE_RADIUS: f32 = 10.0;
pub const TABLE_THICKNESS: f32 = 0.1;
pub const RING_HEIGHT: f32 = 3.0;

#[derive(Component)]
pub struct TablePart;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Table
    commands.spawn((
        TablePart,
        RigidBody::Static,
        Collider::cylinder(TABLE_RADIUS, TABLE_THICKNESS),
        Friction::new(0.9),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(TABLE_RADIUS, TABLE_THICKNESS)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));

    let ring = Extrusion::new(Annulus::new(TABLE_RADIUS, TABLE_RADIUS * 1.1), RING_HEIGHT);

    commands.spawn((
        TablePart,
        RigidBody::Static,
        Collider::trimesh_from_mesh(&ring.into()).unwrap(),
        Friction::new(0.9),
        PbrBundle {
            transform: Transform::from_xyz(0.0, (RING_HEIGHT + TABLE_THICKNESS) / 2.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            mesh: meshes.add(ring),
            material: materials.add(StandardMaterial::from_color(RED)),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, RING_HEIGHT * 3.0, 0.0),
        ..default()
    });
}

pub fn punch_table(
    mut commands: Commands,
    button_input: Res<ButtonInput<MouseButton>>,
    collisions: Res<Collisions>,
    q_table_parts: Query<Entity, With<TablePart>>,
    mut q_dices: Query<(Entity, &mut LinearVelocity), (With<Dice>, Without<InHand>)>,
) {
    if button_input.just_pressed(MouseButton::Right) {
        let mut moved = false;

        for (entity, mut linear_velocity) in &mut q_dices {
            if q_table_parts
                .iter()
                .any(|table_part| collisions.contains(entity, table_part))
            {
                linear_velocity.0 += Vec3::new(0.0, 5.0, 0.0);
                moved = true;
            }
        }

        if moved {
            commands.remove_resource::<LastCombination>();
        }
    }
}
