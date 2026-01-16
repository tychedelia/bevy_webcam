//! A simple example demonstrating webcam texture on a rotating cube.
use bevy::{
    app::AppExit,
    prelude::*,
};

use bevy_webcam::{
    BevyWebcamPlugin,
    WebcamStream,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "bevy_webcam".to_string(),
                    ..default()
                }
                .into(),
                ..default()
            }),
            BevyWebcamPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, press_esc_close))
        .run();
}

#[derive(Component)]
struct RotatingCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    stream: Res<WebcamStream>,
) {
    // Cube with webcam texture
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(stream.frame.clone()),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RotatingCube,
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MeshMaterial3d<StandardMaterial>), With<RotatingCube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut transform, material_handle) in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
        transform.rotate_x(time.delta_secs() * 0.3);

        // Mark material as changed so it picks up the updated webcam texture
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let _ = &mut *material;
        }
    }
}

fn press_esc_close(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
