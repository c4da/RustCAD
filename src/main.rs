//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*,  winit::WinitSettings};
use ui::setup_ui;

mod view;
mod tools;
mod ui;

fn spawn_camera(mut commands: Commands) {
    let mut camera = view::PanOrbitCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(1.0, 2.0, 3.0);
    camera.state.radius = 50.0;
    camera.state.pitch = 15.0f32.to_radians();
    camera.state.yaw = 30.0f32.to_radians();
    commands.spawn(camera);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (
            setup,
            spawn_camera,
            setup_ui,
        ))
        .add_systems(Update, (
            view::pan_orbit_camera
                .run_if(any_with_component::<view::PanOrbitState>),
                ui::button_system,
        ))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    /*commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));*/

}
