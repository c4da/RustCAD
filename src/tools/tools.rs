
//tools module
use bevy::prelude::*;
use bevy::render::mesh::{Mesh, SphereMeshBuilder, SphereKind};
use bevy::pbr::{MaterialMeshBundle, StandardMaterial};
use std::collections::HashSet;

use super::{colors, vec3_rounded::Vec3Rounded};

/*
method takes a ref of a Bevy mesh and returns a map of unique vertices
*/
fn get_vertices(mesh: &Mesh) -> Vec<Vec3> {
    let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(raw_positions)) = 
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
