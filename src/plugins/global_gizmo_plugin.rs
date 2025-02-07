use crate::tools;

use bevy::{prelude::*,
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver}};
use tools::colors;

/// The GizmoPlugin for displaying a transformation gizmo in a Bevy application.
pub struct GlobalGizmoPlugin;

impl Plugin for GlobalGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_gizmo)
            .add_systems(Update, update_gizmo_transform);
    }
}

#[derive(Component)]
pub struct Gizmo;

fn setup_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let translation = Vec3::new(100.0, 100.0, 0.0);

    // World Gizmo (X-axis)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 0.1, 0.1)))), // Create cuboid with exact dimensions
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors::RED,
            emissive: colors::RED.to_linear() * 0.5, // Add some glow
            unlit: true, // Make material independent of lighting
            ..default()})),
        Transform::from_xyz(1.0, 0.0, 0.0).with_translation(translation),
        Gizmo,
        PickingBehavior::IGNORE,
        NotShadowCaster,
        NotShadowReceiver
    ));

    // World Gizmo (Y-axis)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.1, 2.0, 0.1)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors::GREEN,
            emissive: colors::GREEN.to_linear() * 0.5, // Add some glow
            unlit: true, // Make material independent of lighting
            alpha_mode: AlphaMode::Blend,
            ..default()})),
        Transform::from_xyz(0.0, 1.0, 0.0).with_translation(translation),
        Gizmo,
        PickingBehavior::IGNORE,
        NotShadowCaster,
        NotShadowReceiver
    ));

    // World Gizmo (Z-axis)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.1, 0.1, 2.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors::BLUE,
            emissive: colors::BLUE.to_linear() * 0.5, // Add some glow
            unlit: true, // Make material independent of lighting
            ..default()})),
        Transform::from_xyz(0.0, 0.0, 1.0).with_translation(translation),
        Gizmo,
        PickingBehavior::IGNORE,
        NotShadowCaster,
        NotShadowReceiver
    ));
}

// Keep the gizmo fixed relative to the camera
fn update_gizmo_transform(
    mut params: ParamSet<(
        Query<&Transform, With<Camera>>,
        Query<&mut Transform, With<Gizmo>>,
    )>,
) {
    // Get camera transform first
    let camera_transform = if let Ok(transform) = params.p0().get_single() {
        transform.clone()
    } else {
        return;
    };

    // Then update gizmo transforms
    for mut gizmo_transform in params.p1().iter_mut() {
        // Place the gizmo in front of the camera
        gizmo_transform.translation = camera_transform.translation + camera_transform.forward() * 20.0 
        + camera_transform.up() * 7.0 + camera_transform.right() * 7.0; //todo this needs to be fixed so it is set to the right position according the windows size
        // gizmo_transform.rotation = camera_transform.rotation;
    }
}