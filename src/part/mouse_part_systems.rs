use bevy::{color::palettes::tailwind::*, input::mouse::{self, MouseButtonInput}, picking::pointer::PointerInteraction, prelude::*};
use crate::tools::{colors::{PRESSED_COLOR, NO_CHANGE_COLOR, HOVER_COLOR}, components::Shape};
use super::components::{Face, Part};
use crate::ui::ui_button_systems::EditorMode;

pub fn update_materials_system(
    pointers: Query<&PointerInteraction>,
    mut mesh_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Mesh3d)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    buttons: Res<ButtonInput<MouseButton>>,
    selection_mode: Res<EditorMode>, // EditorMode is a custom resource that tracks the current select mode
) {

    // Only allow selection in Select mode
    if *selection_mode != EditorMode::SelectFace {
        return;
    }

    let no_change_matl = materials.add(NO_CHANGE_COLOR);
    let hover_matl = materials.add(HOVER_COLOR);
    let pressed_matl = materials.add(PRESSED_COLOR);
    let mut interacted_entities = Vec::new();

    // First, set all materials to default
    for (mut material, _) in mesh_query.iter_mut() {
        material.0 = no_change_matl.clone();
    }

     // Handle active interactions
     for interaction in pointers.iter() {
        if let Some((entity, hit)) = interaction.get_nearest_hit() {
            interacted_entities.push(entity);
            
            if let Ok((mut material, mesh)) = mesh_query.get_mut(*entity) {
                material.0 = if buttons.pressed(MouseButton::Left) {
                    pressed_matl.clone()
                } else {
                    hover_matl.clone()
                };
                println!("Interacting with mesh: {:?}", mesh.0);
            }
        }
    };
}


pub fn handle_face_selection(
    mouse: Res<ButtonInput<MouseButton>>,
    pointers: Query<&PointerInteraction>,
    mut face_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Face, &Parent)>,
    mut part_query: Query<&mut Part>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selection_mode: Res<EditorMode>,
) {
    // Only process when left mouse button is just pressed
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    if *selection_mode != EditorMode::SelectFace {
        return;
    }
    for interaction in pointers.iter() {
        // Similar to draw_mesh_intersections, get the nearest hit
        if let Some((entity, hit)) = interaction.get_nearest_hit() {
            if let Ok((mut material, face, parent)) = face_query.get_mut(entity.clone()) {
                // Update material
                let pressed_matl = materials.add(PRESSED_COLOR);
                material.0 = pressed_matl.clone();

                // Update selected faces in parent Part
                if let Ok(mut part) = part_query.get_mut(parent.get()) {
                    part.selected_faces.clear();
                    part.selected_faces.push(face.clone());
                    println!("Selected faces: {:?}", part.selected_faces);
                }
            }
        }
    }
}

/// A system that draws hit indicators for every pointer.
pub fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
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
pub fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

/// An observer to rotate an entity when it is dragged
pub fn rotate_on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.entity()).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}
