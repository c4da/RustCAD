mod tools;
mod ui;
mod view;
mod part;
mod plugins;

use std::f32::consts::PI;

use crate::plugins::global_gizmo_plugin::GlobalGizmoPlugin;
use bevy::{prelude::*, color::palettes::css::*};
use tools::colors;
use part::components::ExtrusionParams;
use part::components::Part;
use ui::{ui_elements::ToolbarAction, EditorMode};
use part::mouse_part_systems::{handle_face_selection, update_materials_system, draw_mesh_intersections};
use view::PanOrbitCamera;

#[derive(Component)]
struct CameraGizmo;

// inspector for debugging
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource, Default)]
struct GizmoState {
    rotation: Quat,
}

fn main() {
    App::new()
        // MeshPickingPlugin is not a default plugin
        .add_plugins((DefaultPlugins, MeshPickingPlugin, GlobalGizmoPlugin))
        .add_event::<ToolbarAction>()
        .insert_resource(ExtrusionParams {
            direction: Vec3::Y,
            distance: 1.0,
        })
        .init_gizmo_group::<MyRoundGizmos>()
        .init_resource::<EditorMode>()
        .init_resource::<GizmoState>()
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, 
        (setup_scene, 
                ui::setup_ui,
            ))
        .add_systems(Update, (
            view::pan_orbit_camera.run_if(any_with_component::<view::PanOrbitCamera>),
            ui::button_highlight_system,
            part::draw_mesh_intersections,
            part::handle_face_selection,
            part::update_materials_system,
            part::rotate,
            ui::button_action_system,
            ui::handle_toolbar_actions,
            ui::update_selection_mode_buttons,
            draw_gizmos,
            // update_gizmo_transform.after(part::rotate), // Run after rotate to avoid conflicts
        ))
        .run();
}

// #[derive(Component)]
// struct Gizmo;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>, 
    mut gizmos: Gizmos,
) {
    let points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
    ];

    part::create_3d_object_system(&mut commands, &mut meshes, &mut materials, points);
    let no_change_matl = materials.add(colors::NO_CHANGE_COLOR);
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(no_change_matl.clone()),
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

    // Camera that can be panned and orbited
    let (camera_bundle, pan_orbit) = view::spawn_camera();
    commands.spawn((
        camera_bundle,
        pan_orbit,
    ));

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

    // let translation = Vec3::new(100.0, 100.0, 0.0);

    // // World Gizmo (X-axis)
    // commands.spawn((
    //     Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 0.1, 0.1)))), // Create cuboid with exact dimensions
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: colors::RED,
    //         emissive: colors::RED.to_linear() * 0.5, // Add some glow
    //         unlit: true, // Make material independent of lighting
    //         ..default()})),
    //     Transform::from_xyz(1.0, 0.0, 0.0).with_translation(translation),
    //     Gizmo,
    //     PickingBehavior::IGNORE,
    // ));

    // // World Gizmo (Y-axis)
    // commands.spawn((
    //     Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.1, 2.0, 0.1)))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: colors::GREEN,
    //         emissive: colors::GREEN.to_linear() * 0.5, // Add some glow
    //         unlit: true, // Make material independent of lighting
    //         alpha_mode: AlphaMode::Blend,
    //         ..default()})),
    //     Transform::from_xyz(0.0, 1.0, 0.0).with_translation(translation),
    //     Gizmo,
    //     PickingBehavior::IGNORE,
    // ));

    // // World Gizmo (Z-axis)
    // commands.spawn((
    //     Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.1, 0.1, 2.0)))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: colors::BLUE,
    //         emissive: colors::BLUE.to_linear() * 0.5, // Add some glow
    //         unlit: true, // Make material independent of lighting
    //         ..default()})),
    //     Transform::from_xyz(0.0, 0.0, 1.0).with_translation(translation),
    //     Gizmo,
    //     PickingBehavior::IGNORE,
    // ));
}

// // Keep the gizmo fixed relative to the camera
// fn update_gizmo_transform(
//     mut params: ParamSet<(
//         Query<&Transform, With<Camera>>,
//         Query<&mut Transform, With<Gizmo>>,
//     )>,
// ) {
//     // Get camera transform first
//     let camera_transform = if let Ok(transform) = params.p0().get_single() {
//         transform.clone()
//     } else {
//         return;
//     };

//     // Then update gizmo transforms
//     for mut gizmo_transform in params.p1().iter_mut() {
//         // Place the gizmo in front of the camera
//         gizmo_transform.translation = camera_transform.translation + camera_transform.forward() * 20.0 
//         + camera_transform.up() * 7.0 + camera_transform.right() * 7.0; //todo this needs to be fixed so it is set to the right position according the windows size
//         // gizmo_transform.rotation = camera_transform.rotation;
//     }
// }

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn draw_gizmos(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    _time: Res<Time>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    windows: Query<&Window>,
) {
    // Draw orientation gizmo in top-left corner
    if let (Ok((camera, camera_transform)), Ok(window)) = (camera_q.get_single(), windows.get_single()) {
        let screen_pos = Vec2::new(
            100.0,  // 100 pixels from left
            100.0,  // 100 pixels from top
        );
        
        if let Ok(ray) = camera.viewport_to_world(camera_transform, screen_pos) {
            let distance = 5.0;
            let gizmo_pos = ray.origin + ray.direction * distance;
            // Use identity rotation to keep axes aligned with world space
            let gizmo_transform = Transform::from_translation(gizmo_pos).with_rotation(Quat::IDENTITY);
            gizmos.axes(gizmo_transform, 0.3);
        }
    }

    // Draw world origin marker
    gizmos.cross(Vec3::new(0., 0., 0.), 0.5, FUCHSIA);
    gizmos.grid(
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(20),
        Vec2::new(2., 2.),
        // Light gray
        LinearRgba::gray(0.65),
    );
}

// fn update_gizmo_state(
//     mut gizmo_state: ResMut<GizmoState>,
//     part_q: Query<(&Part, &GlobalTransform)>,
// ) {
//     // Update rotation from part
//     if let Some((_, transform)) = part_q.iter().next() {
//         gizmo_state.rotation = transform.rotation();
//     }
// }
