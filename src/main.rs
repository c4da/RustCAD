mod tools;
mod ui;
mod view;
mod part;

use std::f32::consts::PI;

use bevy::{prelude::*, color::palettes::css::*};
use tools::colors;
use part::components::ExtrusionParams;
use part::components::Part;
use ui::{ui_elements::ToolbarAction, EditorMode};
use part::mouse_part_systems::{handle_face_selection, update_materials_system};

// inspector for debugging
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource, Default)]
struct GizmoState {
    rotation: Quat,
}

fn main() {
    App::new()
        // MeshPickingPlugin is not a default plugin
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
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
            view::pan_orbit_camera.run_if(any_with_component::<view::PanOrbitState>),
            ui::button_highlight_system,
            part::draw_mesh_intersections,
            part::handle_face_selection,
            part::update_materials_system,
            part::rotate,
            ui::button_action_system,
            ui::handle_toolbar_actions,
            ui::update_selection_mode_buttons,
            draw_gizmos,
            update_gizmo_state,
            draw_global_axes,
        ))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>, 
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

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn draw_gizmos(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    _time: Res<Time>,
) {
    gizmos.cross(Vec3::new(0., 0., 0.), 0.5, FUCHSIA);
    gizmos.grid(
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(20),
        Vec2::new(2., 2.),
        // Light gray
        LinearRgba::gray(0.65),
    );
}

fn update_gizmo_state(
    mut gizmo_state: ResMut<GizmoState>,
    part_q: Query<(&Part, &GlobalTransform)>,
) {
    // Update rotation from part
    if let Some((_, transform)) = part_q.iter().next() {
        gizmo_state.rotation = transform.rotation();
    }
}

fn draw_global_axes(
    mut gizmos: Gizmos,
    gizmo_state: Res<GizmoState>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    windows: Query<&Window>,
) {
    if let (Ok((camera, camera_transform)), Ok(window)) = (camera_q.get_single(), windows.get_single()) {
        // Fixed screen position (top-right corner)
        let screen_pos = Vec2::new(
            window.width() - 140.0,  // 100 pixels from right
            110.0,  // 100 pixels from top
        );
        
        // Convert screen position to world space
        if let Ok(ray) = camera.viewport_to_world(camera_transform, screen_pos) {
            // Position the gizmo along the ray at a fixed distance
            let distance = 5.0;
            let gizmo_pos = ray.origin + ray.direction * distance;
            
            let gizmo_transform = GlobalTransform::from(
                Transform::from_translation(gizmo_pos)
                    .with_rotation(gizmo_state.rotation)
            );
            
            // Draw axes with fixed size
            gizmos.axes(gizmo_transform, 0.5);
        }
    }
}

// fn draw_local_axes(
//     mut gizmos: Gizmos,
//     camera_q: Query<&GlobalTransform, With<Camera>>,
//     gizmo_state: Res<GizmoState>,
// ) {
//     if let Ok(camera_transform) = camera_q.get_single() {
//         // Calculate a fixed offset from the camera
//         let forward = camera_transform.forward();
//         let right = camera_transform.right();
//         let up = camera_transform.up();
        
//         // Position the gizmo at a fixed distance in front of the camera, offset to the top-right
//         let offset = -forward * 5.0 + right * 2.0 + up * 2.0;
//         let gizmo_position = offset;
//         // let gizmo_position = camera_transform.translation() + offset;
        
//         // Create transform for gizmo using the stored rotation
//         let gizmo_transform = GlobalTransform::from(
//             Transform::from_translation(gizmo_position)
//                 .with_rotation(gizmo_state.rotation)
//         );
        
//         // Draw axes with fixed size
//         gizmos.axes(gizmo_transform, 1.0);
//     }
// }
