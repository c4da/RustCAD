
use bevy::prelude::*;
use tools::colors;


// inspector for debugging
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod tools;
mod ui;
mod view;
mod part;

fn main() {
    App::new()
        // MeshPickingPlugin is not a default plugin
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
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
                part::rotate,))
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

    commands.spawn((
        AudioPlayer::new(
        asset_server.load("resources/V-Background.ogg")
        ),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        },)
    );
}
