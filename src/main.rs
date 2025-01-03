//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    prelude::*,
    // winit::WinitSettings,
    picking::pointer::PointerInteraction,
};

use tools::colors::{HOVER_COLOR, NO_CHANGE_COLOR, PRESSED_COLOR, WHITE};
// use bevy_mod_picking::{
//     PickingCameraBundle, PickingPlugin, PickableMesh, MeshPickingPlugin,
//     PointerClick, OnPointerDown,
// };

// use tools::tools::StoredVertices;
use tools::colors::*;

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
    // camera.far = 1000.0;
    commands.spawn(camera);
}

fn main() {
    App::new()
        // .init_resource::<StoredVertices>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        // .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (
            setup,
            spawn_camera,
            ui::setup_ui,
            ))
        .add_systems(
            Update, (
            view::pan_orbit_camera
                            .run_if(any_with_component::<view::PanOrbitState>),
            ui::button_highlight_system,
            draw_mesh_intersections, 
            rotate,
            // tools::tools::create_vertex_dummies, //make create vertex dummies run every frame so it has access to system resources
        ))
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component)]
struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

#[derive(Component)]
struct Mesh3d(Handle<Mesh>);

#[derive(Component)]
struct MeshMaterial3d(Handle<StandardMaterial>);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut stored: ResMut<StoredVertices>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(400.0))),
        MeshMaterial3d(materials.add(GRAY)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        // GlobalTransform::default(),
        Visibility::Visible,
        PickingBehavior::IGNORE, // Disable picking for the ground plane.
    ));

    // cube
    commands.spawn((
        // Mesh3d(mesh_handle),
        // MeshMaterial3d(materials.add(WHITE)),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        // GlobalTransform::default(),
        Visibility::Visible,
        Shape,
    ))
    .observe(handle_mouse_over_object::<Pointer<Over>>(materials.add(NO_CHANGE_COLOR).clone()))
    .observe(handle_mouse_over_object::<Pointer<Out>>(materials.add(WHITE).clone()))
    .observe(handle_mouse_over_object::<Pointer<Down>>(materials.add(PRESSED_COLOR).clone()))
    .observe(handle_mouse_over_object::<Pointer<Up>>(materials.add(HOVER_COLOR).clone()))
    .observe(rotate_on_drag);

    commands.spawn((
        Mesh3d( meshes.add(Cuboid::new(10.0, 10.0, 10.0))),
        MeshMaterial3d(materials.add(RED)),
        Transform::from_xyz(20.0, 10.5, 0.0),
        // GlobalTransform::default(),
        Visibility::Visible,
        Shape,
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        // GlobalTransform::default(),
    ));

    // // Ground
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
    //     MeshMaterial3d(materials.add(Color::from(GRAY_300))),
    //     PickingBehavior::IGNORE, // Disable picking for the ground plane.
    // ));

    // // Light
    // commands.spawn((
    //     PointLight {
    //         shadows_enabled: true,
    //         intensity: 10_000_000.,
    //         range: 100.0,
    //         shadow_depth_bias: 0.2,
    //         ..default()
    //     },
    //     Transform::from_xyz(8.0, 16.0, 8.0),
    // ));

    // // Camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    // ));

}

// fn handle_mesh_click(
//     mut events: EventReader<Pointer<Click>>,
//     query: Query<&Mesh3d>,
//     meshes: Res<Assets<Mesh>>,
//     mut stored: ResMut<StoredVertices>,
// ) {
//     for event in events.read() {
//         if event.button == MouseButton::Left {
//             if let Ok(mesh_handle) = query.get(event.entity) {
//                 if let Some(mesh) = meshes.get(&mesh_handle.0) {
//                     let vertices = tools::tools::get_vertices(mesh.clone());
//                     stored.store_and_flag(vertices);
//                 }
//             }
//         }
//     }
// }

fn handle_mouse_over_object<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<(&mut MeshMaterial3d, &Mesh3d)>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok((mut material, mesh)) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
            println!("Clicked mesh: {:?}", mesh.0);
            // Now you have access to mesh.0 which is the Handle<Mesh> of the clicked object
            // You can use this to work with the mesh data
        }
    }
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED);
        gizmos.arrow(point, point + normal.normalize() * 0.5, RED);
    }
}

/// A system that rotates all shapes.
fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

/// An observer to rotate an entity when it is dragged
fn rotate_on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.entity()).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}