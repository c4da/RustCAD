use bevy::{color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*};
use crate::tools::{components::Shape};
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

pub fn face_selection<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, (Query<(&mut MeshMaterial3d<StandardMaterial>, &Face, &Parent)>, Query<&mut Part>)) {
    move |trigger, (mut face_query, mut part_query)| {
        if let Ok((mut material, face, parent)) = face_query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
            println!("Clicked face: {:?}", face.get_vertices());
            
            // Get the actual Part component from the parent entity
            if let Ok(mut part) = part_query.get_mut(parent.get()) {
                part.selected_faces.clear();
                part.selected_faces.push(face.clone());
                println!("Selected faces: {:?}", part.selected_faces);
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
