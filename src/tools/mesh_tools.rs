
//tools module
use bevy::{
    prelude::*,
    pbr::StandardMaterial,
    render::mesh::{Mesh, VertexAttributeValues},
};
use std::collections::HashSet;

use super::{colors, vec3_rounded::Vec3Rounded,};

#[derive(Resource)]
pub struct ToolResources {
    pub material_handle: Handle<StandardMaterial>,
    pub mesh_handle: Handle<Mesh>,
}

/*
method takes a ref of a Bevy mesh and returns a map of unique vertices
*/
pub fn get_vertices(mesh: &Mesh) -> Vec<Vec3> {
    let Some(VertexAttributeValues::Float32x3(raw_positions)) = 
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        return vec![];
    };

    let mut unique = HashSet::new();
    for pos in raw_positions {
        let rounded = Vec3Rounded::from(pos.clone());
        unique.insert(rounded);
    }

    unique.into_iter().map(|x| x.to_vec3()).collect()
}

pub fn create_vertex_dummies(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>, points: &Vec<Vec3>) {

    let sphere_mesh = meshes.add(Sphere::new(0.05)
        .mesh()
        .ico(7)
        .unwrap());

    
    let sphere_material = materials.add(StandardMaterial {
        base_color: colors::YELLOW,
        ..default()
    });

    for pos in points {
        commands.spawn((
            Mesh3d(sphere_mesh.clone()),
            MeshMaterial3d(sphere_material.clone()),
            Transform::from_translation(*pos),
        ));        
    }
}


// A system that draws hit indicators for every pointer. 
// to do get this working
// pub fn transform_mouse_pointer_to_vect<'a>(
//     pointers: &'a Query<&PointerInteraction>
// ) -> impl Iterator<Item = (bevy::prelude::Vec3, bevy::prelude::Vec3)> + 'a + use<'a, '_> {
//     pointers
//         .iter()
//         .filter_map(|interaction| interaction.get_nearest_hit())
//         .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
// }