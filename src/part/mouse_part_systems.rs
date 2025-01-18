use bevy::{color::palettes::tailwind::*, input::mouse::{self, MouseButtonInput}, picking::pointer::PointerInteraction, prelude::*};
use crate::{tools::{colors::{HOVER_COLOR, NO_CHANGE_COLOR, PRESSED_COLOR}, components::Shape}, plugins::global_gizmo_plugin::Gizmo};
use super::components::{Face, Part};
use crate::ui::ui_button_systems::EditorMode;

pub fn update_materials_system(
    pointers: Query<&PointerInteraction>,
    mut mesh_query: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>, &Face, &Parent)>,
    mut part_query: Query<&Part>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selection_mode: Res<EditorMode>,
) {
    if *selection_mode != EditorMode::SelectFace {
        return;
    }

    let no_change_matl = materials.add(NO_CHANGE_COLOR);
    let hover_matl = materials.add(HOVER_COLOR);
    let pressed_matl = materials.add(PRESSED_COLOR);

    // First set all materials to their default state
    for (entity, mut material, face, parent) in mesh_query.iter_mut() {
        if let Ok(part) = part_query.get(parent.get()) {
            // Check if this face is selected
            let is_selected = part.selected_faces.iter().any(|selected_face| selected_face == face);
            material.0 = if is_selected {
                pressed_matl.clone()
            } else {
                no_change_matl.clone()
            };
        }
    }

    // Then handle hover states
    for interaction in pointers.iter() {
        if let Some((hovered_entity, _)) = interaction.get_nearest_hit() {
            if let Ok((_, mut material, face, parent)) = mesh_query.get_mut(*hovered_entity) {
                if let Ok(part) = part_query.get(parent.get()) {
                    // Only show hover if not selected
                    if !part.selected_faces.iter().any(|selected_face| selected_face == face) {
                        material.0 = hover_matl.clone();
                    }
                }
            }
        }
    }
}


pub fn handle_face_selection(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    pointers: Query<&PointerInteraction>,
    face_query: Query<(&Face, &Parent)>,
    mut part_query: Query<&mut Part>,
    selection_mode: Res<EditorMode>,
) {
    if *selection_mode != EditorMode::SelectFace {
        return;
    }

    // Only process when left mouse button is just pressed
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let multi_select = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    for interaction in pointers.iter() {
        if let Some((entity, _)) = interaction.get_nearest_hit() {
            if let Ok((face, parent)) = face_query.get(*entity) {
                if let Ok(mut part) = part_query.get_mut(parent.get()) {
                    // If not multi-selecting, clear previous selections
                    if !multi_select {
                        part.selected_faces.clear();
                        part.selected_faces.push(face.clone());
                    } else {
                        // In multi-select mode, toggle the face selection
                        if let Some(index) = part.selected_faces.iter().position(|f| f == face) {
                            part.selected_faces.remove(index);
                        } else {
                            part.selected_faces.push(face.clone());
                        }
                    }
                    println!("Selected faces count: {}", part.selected_faces.len());
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
