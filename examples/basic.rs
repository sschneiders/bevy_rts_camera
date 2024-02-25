//! Demonstrates the simplest usage

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rts_camera::{
    RtsCamera, RtsCameraEye, RtsCameraLock, RtsCameraPlugin, RtsCameraSystemSet,
};
use std::f32::consts::TAU;

mod stepping;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RtsCameraPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        // .add_plugins(
        //     stepping::SteppingPlugin::default()
        //         .add_schedule(Update)
        //         .at(Val::Percent(35.0), Val::Percent(50.0)),
        // )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_unit, toggle_lock, swap_cameras).before(RtsCameraSystemSet),
        )
        .run();
}

#[derive(Component)]
struct Move;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // Some "terrain"
    let terrain_material = materials.add(Color::rgb(0.8, 0.7, 0.6));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 0.5, 1.0)),
        material: terrain_material.clone(),
        transform: Transform::from_xyz(0.0, 0.25, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(3.0, 0.2, 1.0)),
        material: terrain_material.clone(),
        transform: Transform::from_xyz(3.0, 0.1, -1.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(2.0, 0.3, 3.0)),
        material: terrain_material.clone(),
        transform: Transform::from_xyz(-3.0, 0.15, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(3.0)),
        material: terrain_material.clone(),
        transform: Transform::from_xyz(-5.0, 0.0, 3.0),
        ..default()
    });
    // A moving unit
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.1, 0.3)),
            material: terrain_material.clone(),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        })
        .insert(Move)
        .insert(RtsCameraLock);
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_translation(Vec3::new(0.0, 2.5, 1.5)),
                ..default()
            },
            RtsCamera::default(),
        ))
        .with_children(|parent| {
            parent.spawn((Camera3dBundle::default(), RtsCameraEye));
        });
    // Debug Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(8.0, 1.0, 14.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        },
        PanOrbitCamera {
            enabled: false,
            zoom_sensitivity: 0.0,
            ..default()
        },
    ));
}

/// Move the cube in a circle around the Y axis
fn animate_unit(
    time: Res<Time>,
    mut cube_q: Query<&mut Transform, With<Move>>,
    mut angle: Local<f32>,
) {
    if let Ok(mut cube_tfm) = cube_q.get_single_mut() {
        // Rotate 20 degrees a second, wrapping around to 0 after a full rotation
        *angle += 20f32.to_radians() * time.delta_seconds() % TAU;
        // Convert angle to position
        let pos = Vec3::new(angle.sin() * 1.5, 0.25, angle.cos() * 1.5);
        cube_tfm.translation = pos;
    }
}

fn toggle_lock(
    mut commands: Commands,
    mut cube_q: Query<Entity, With<Move>>,
    lock_q: Query<&RtsCameraLock, With<Move>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::KeyL) {
        if let Ok(cube) = cube_q.get_single_mut() {
            if lock_q.is_empty() {
                commands.entity(cube).insert(RtsCameraLock);
            } else {
                commands.entity(cube).remove::<RtsCameraLock>();
            }
        }
    }
}

fn swap_cameras(
    mut orbit_cam: Query<(&mut Camera, &mut PanOrbitCamera)>,
    mut rts_cam: Query<&mut Camera, (With<RtsCameraEye>, Without<PanOrbitCamera>)>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    if button_input.just_pressed(KeyCode::Space) {
        let (mut orbit_camera, mut orbit_cam) = orbit_cam.get_single_mut().unwrap();
        let mut rts_cam = rts_cam.get_single_mut().unwrap();
        orbit_camera.is_active = !orbit_camera.is_active;
        orbit_cam.enabled = orbit_camera.is_active;
        rts_cam.is_active = !rts_cam.is_active;
    }
}
