use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};

pub const TABLE_RADIUS: f32 = 10.0;
pub const TABLE_THICKNESS: f32 = 0.1;
pub const RING_HEIGHT: f32 = 3.0;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Table
    commands.spawn((
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
        RigidBody::Static,
        Collider::trimesh_from_mesh(&ring.into()).unwrap(),
        Friction::new(0.9),
        PbrBundle {
            transform: Transform::from_xyz(0.0, RING_HEIGHT / 2.0, 0.0)
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

    commands.spawn((
        crate::camera::FlyCam,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, TABLE_RADIUS * 1.5, TABLE_RADIUS * 1.5)
                .looking_at(Vec3::ZERO, Dir3::Y),
            ..default()
        },
    ));
}
