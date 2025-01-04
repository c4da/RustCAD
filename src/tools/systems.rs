// use bevy::prelude::*;
// use super::components::{Mesh3d, MeshMaterial3d, VisibilityBundle};

// pub fn convert_components(
//     mut commands: Commands,
//     query: Query<(Entity, &Mesh3d, &MeshMaterial3d), Added<Mesh3d>>,
// ) {
//     for (entity, mesh, material) in query.iter() {
//         commands.entity(entity).insert((
//             mesh.0.clone(),
//             material.0.clone(),
//             VisibilityBundle::default(),
//         ));
//     }
// }
