use bevy::{color::palettes::tailwind::*, input::mouse::{self, MouseButtonInput}, picking::pointer::PointerInteraction, prelude::*};
use crate::tools::{colors::{PRESSED_COLOR, NO_CHANGE_COLOR, HOVER_COLOR}, components::Shape};
use super::components::{Face, Part};

// Returns an observer that updates the entity's material and provides access to its mesh.
pub fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<(&mut MeshMaterial3d<StandardMaterial>, &Mesh3d)>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        // The query accesses entities with both MeshMaterial3d<StandardMaterial> and Mesh3d components
        if let Ok((mut material, mesh)) = query.get_mut(trigger.entity()) {
            // `material` is a mutable reference to the MeshMaterial3d<StandardMaterial> component
            // `mesh` is an immutable reference to the Mesh3d component
            material.0 = new_material.clone();
            println!("Clicked mesh: {:?}", mesh.0);
            // Now you have access to mesh.0 which is the Handle<Mesh> of the clicked object
            // You can use this to work with the mesh data
        }
    }
}

pub fn update_materials_system(
    pointers: Query<&PointerInteraction>,
    mut mesh_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Mesh3d)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
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
) {
    // Only process when left mouse button is just pressed
    if !mouse.pressed(MouseButton::Left) {
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
