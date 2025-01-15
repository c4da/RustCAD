use std::any::Any;
use bevy::{color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*};
use crate::tools::{colors::{PRESSED_COLOR, NO_CHANGE_COLOR, HOVER_COLOR}, components::Shape};
use super::components::{Face, Part};
use crate::ui::ui_button_systems::EditorMode;
// use std::sync::{LazyLock, Mutex};


// mut Face selected_face;
// static SELECTED_FACE: LazyLock<Mutex<Option<Entity>>> = LazyLock::new(|| Mutex::new(None));

pub fn update_materials_system(
    pointers: Query<&PointerInteraction>,
    mut mesh_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Mesh3d, &mut Face)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut part_query: Query<&mut Part>,
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
    for (mut material, _, part) in mesh_query.iter_mut() {
        if !part.selected {
            material.0 = no_change_matl.clone();
        }
    }

     // Handle active interactions
     for interaction in pointers.iter() {
        if let Some((entity, _hit)) = interaction.get_nearest_hit() {
            interacted_entities.push(entity);
            
            if let Ok((mut material, mesh, mut part)) = mesh_query.get_mut(*entity) {
                // Check selected faces
                // if let Ok(mut part) = part_query.get_mut(parent.get()) {
                //     println!("Some {:?}", part.selected_faces);
                // }

                /*material.0 = */if buttons.pressed(MouseButton::Left) {
                    warn!("Pressed");
                    // SELECTED_FACE.lock().unwrap().replace(*entity);
                    part.selected = true;
                    material.0 = pressed_matl.clone()
                } else if !part.selected {
                    warn!("Hover {:?} {:?}", material.0, pressed_matl.clone());
                    material.0 = hover_matl.clone()
                } else {
                    warn!("Should stay pressed");
                    material.0 = pressed_matl.clone();
                };
                println!("Interacting with mesh: {:?} {:?}", mesh.0, interacted_entities.len());
            }
        }
    };
}


pub fn handle_face_selection(
    mouse: Res<ButtonInput<MouseButton>>,
    pointers: Query<&PointerInteraction>,
    mut face_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut Face, &Parent)>,
    mut part_query: Query<&mut Part>,
    // mut mat_query: Query<(&mut MeshMaterial3d<StandardMaterial>, Without<&Face>)>,
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
    warn!("handle_face_selection:85");
    
    for mut part in part_query.iter_mut() {
        warn!("Itera {:?}", part);
        if part.selected_faces.len() > 0 {
            
            warn!("Unslect {:?}", part);
            for face in part.selected_faces.iter_mut() {
                
                if face.selected {
                    warn!("Faces {:?}", face);
                    face.selected = false;
                }
            }
            // part.selected_faces.remove(0).selected = false;
        }
    }
    for interaction in pointers.iter() {
        // Similar to draw_mesh_intersections, get the nearest hit
        if let Some((entity, hit)) = interaction.get_nearest_hit() {
            // {
            //     println!("Gzu {}", entity);
            //     // I have an entity
            //     // Get
            //     // if let Ok((mut material, face, parent)) = face_query.get_mut(entity.clone()) {
            //     //
            //     //     if let Ok(mut part) = part_query.get_mut(parent.get()) {
            //     //         // entity2 = part.selected_faces.get(0);
            //     //         if part.selected_faces.len() > 0 {
            //     //             let entity2 = part.selected_faces.remove(0).entity;
            //     //
            //     //             if let Some(entity2) = entity2 {
            //     //                 println!("Gzu {} {}", entity, entity2)
            //     //                 // if let Ok((mut material2, _, _2)) = face_query.get_mut(entity2.clone()) {
            //     //                 //     let no_change_matl = materials.add(NO_CHANGE_COLOR);
            //     //                 //     material2.0 = no_change_matl.clone();
            //     //                 // }
            //     //             }
            //     //         }
            //     //     }
            //     // }
            // }
            if let Ok((mut material, mut face, parent)) = face_query.get_mut(entity.clone()) {
                // Update material
                let pressed_matl = materials.add(PRESSED_COLOR);
                material.0 = pressed_matl.clone();

                // Update selected faces in parent Part
                if let Ok(mut part) = part_query.get_mut(parent.get()) {
                    // if part.selected_faces.len() > 0 {
                    //     println!("Unselecting");
                    //     part.selected_faces.remove(0).selected = false;
                    // }

                    println!("Clearing");
                    part.selected_faces.clear();
                    face.selected = true;
                    part.selected_faces.push(face.clone());// We are passing a clone... Beware...
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
