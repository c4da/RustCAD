
use std::f32::consts::PI;

use bevy::{color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use tools::components::Shape;

mod tools;
mod ui;
mod view;

fn main() {
    App::new()
        // MeshPickingPlugin is not a default plugin
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, 
            (setup_scene, ui::setup_ui,))
        .add_systems(Update, 
            (view::pan_orbit_camera
                .run_if(any_with_component::<view::PanOrbitState>),
                ui::button_highlight_system, draw_mesh_intersections, rotate,))
        .run();
}

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Set up the materials.
    let white_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));
    let pressed_matl = materials.add(Color::from(YELLOW_300));

       // cube
       commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 10.0, 10.0))),
        MeshMaterial3d(white_matl.clone()),
        // Transform::from_xyz(0.0, 0.5, 0.0),
        Transform::from_xyz(
            0.0,
            10.0,
            Z_EXTENT / 2.,
        ),
        Shape,
    ))
    .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
    .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
    .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
    .observe(rotate_on_drag);

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(ground_matl.clone()),
        PickingBehavior::IGNORE, // Disable picking for the ground plane.
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Static camera for testing
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(0.0, 7.0, 40.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    // ));

    // Camera that can be panned and orbited
    commands.spawn((
                view::spawn_camera(),
                Transform::from_xyz(0.0, 7.0, 40.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),));

    // Instructions
    commands.spawn((
        Text::new("Hover over the shapes to pick them\nDrag to rotate"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Percent(50.0),
            ..default()
        },
    ));
}

/// Returns an observer that updates the entity's material and provides access to its mesh.
fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<(&mut MeshMaterial3d<StandardMaterial>, &Mesh3d)>) {
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
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
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
