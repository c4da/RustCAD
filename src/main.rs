mod tools;
mod ui;
mod view;
mod part;

use std::f32::consts::PI;

use bevy::{prelude::*, color::palettes::css::*};
use tools::colors;
use part::components::ExtrusionParams;
use ui::{ui_elements::ToolbarAction, EditorMode};
// use part::components::ExtrusionParams;
use part::mouse_part_systems::{handle_face_selection, update_materials_system};

// inspector for debugging
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, 
        (setup_scene, 
                ui::setup_ui,
            ))
        .add_systems(Update, 
            (view::pan_orbit_camera
                .run_if(any_with_component::<view::PanOrbitState>),
                ui::button_highlight_system, 
                part::draw_mesh_intersections, 
                part::handle_face_selection,
                part::update_materials_system,
                part::rotate,
                ui::button_action_system,
                ui::handle_toolbar_actions,
                ui::update_selection_mode_buttons,
                draw_gizmos,))
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

    // commands.spawn((
    //     AudioPlayer::new(
    //     asset_server.load("resources/V-Background.ogg")
    //     ),
    //     PlaybackSettings {
    //         mode: bevy::audio::PlaybackMode::Loop,
    //         ..default()
    //     },)
    // );
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn draw_gizmos(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    time: Res<Time>,
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
