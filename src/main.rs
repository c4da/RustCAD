//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use bevy_mod_picking::events::Click;
// use bevy_mod_picking::{
//     PickingCameraBundle, PickingPlugin, PickableMesh, MeshPickingPlugin,
//     PointerClick, OnPointerDown,
// };
use ui::setup_ui;
use tools::tools::StoredVertices;

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
    commands.spawn((
        camera,
        // PickingCameraBundle::default(),
    ));
}

fn main() {
    App::new()
        .init_resource::<StoredVertices>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, 
            (setup,
            spawn_camera,
            ui::setup_ui,
            view::pan_orbit_camera.run_if(any_with_component::<view::PanOrbitState>),
            ))
        .add_systems(Update, (
            ui::button_highlight_system,
            handle_mesh_click,
            tools::tools::create_vertex_dummies, //make create vertex dummies run every frame so it has access to system resources
        ))
        .run();
}

#[derive(Component)]
struct Mesh3d(Handle<Mesh>);

#[derive(Component)]
struct MeshMaterial3d(Handle<StandardMaterial>);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut stored: ResMut<StoredVertices>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        GlobalTransform::default(),
        Visibility::default(),
    ));

    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    // cube
    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(materials.add(Color::rgb(0.486, 0.564, 1.0))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
    ));
}

fn handle_mesh_click(
    mut events: EventReader<Pointer<Click>>,
    query: Query<&Mesh3d>,
    meshes: Res<Assets<Mesh>>,
    mut stored: ResMut<StoredVertices>,
) {
    for event in events.read() {
        if event.button == MouseButton::Left {
            if let Ok(mesh_handle) = query.get(event.entity) {
                if let Some(mesh) = meshes.get(&mesh_handle.0) {
                    let vertices = tools::tools::get_vertices(mesh.clone());
                    stored.store_and_flag(vertices);
                }
            }
        }
    }
}
