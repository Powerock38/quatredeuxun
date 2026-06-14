use avian3d::prelude::*;
use bevy::prelude::*;

use crate::dice::{Dice, InHand};

pub const TRAY_RADIUS: f32 = 10.0;
pub const TRAY_THICKNESS: f32 = 0.1;
pub const TRAY_RING_HEIGHT: f32 = 3.0;

#[derive(Component)]
pub struct TablePart;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Table
    commands.spawn((
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        Transform::from_xyz(0.0, -10.0, 0.0).with_scale(Vec3::splat(10.0)),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("table.glb"))),
    ));

    // Dice tray
    commands.spawn((
        TablePart,
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        Friction::new(0.9),
        Mesh3d(meshes.add(Cylinder::new(TRAY_RADIUS, TRAY_THICKNESS))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // dice tray ring
    commands.spawn((
        TablePart,
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        Friction::new(0.9),
        Transform::from_xyz(0.0, f32::midpoint(TRAY_RING_HEIGHT, TRAY_THICKNESS), 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        Mesh3d(meshes.add(Extrusion::new(
            Annulus::new(TRAY_RADIUS, TRAY_RADIUS * 1.1),
            TRAY_RING_HEIGHT,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(LinearRgba::new(
            0.9, 0.2, 0.1, 0.5,
        )))),
    ));

    // Light
    commands.spawn((
        Transform::from_xyz(0.0, TRAY_RING_HEIGHT * 3.0, 0.0),
        PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            ..default()
        },
    ));
}

pub fn punch_table(
    button_input: Res<ButtonInput<KeyCode>>,
    collisions: Collisions,
    q_table_parts: Query<Entity, With<TablePart>>,
    mut q_dices: Query<(Entity, &mut LinearVelocity), (With<Dice>, Without<InHand>)>,
    q_children: Query<&Children>,
) {
    if button_input.just_pressed(KeyCode::Space) {
        for (entity, mut linear_velocity) in &mut q_dices {
            if q_table_parts.iter().any(|table_part| {
                q_children
                    .iter_descendants(entity)
                    .any(|c| collisions.contains(c, table_part))
            }) {
                linear_velocity.0 += Vec3::new(0.0, 5.0, 0.0);
            }
        }
    }
}
