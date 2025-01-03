
//tools module
use bevy::prelude::*;
use bevy::render::mesh::{Mesh};
use bevy::pbr::{StandardMaterial};
use std::collections::HashSet;

use super::{colors, vec3_rounded::Vec3Rounded};

#[derive(Resource, Default)]
pub struct StoredVertices {
    vertices: Vec<Vec3>,
    needs_dummies: bool,
}

impl StoredVertices {
    pub fn store_and_flag(&mut self, vertices: Vec<Vec3>) {
        self.vertices = vertices;
        self.needs_dummies = true;
    }
}

/*
method takes a ref of a Bevy mesh and returns a map of unique vertices
*/
pub fn get_vertices(mesh: Mesh) -> Vec<Vec3> {
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

pub fn create_vertex_dummies(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stored: ResMut<StoredVertices>,
) {
    if !stored.needs_dummies {
        return;
    }

    let sphere_mesh = meshes.add(Sphere::new(0.05)
        .mesh()
        .ico(7)
        .unwrap());

    let sphere_material = materials.add(StandardMaterial {
        base_color: colors::YELLOW,
        ..default()
    });

    for pos in stored.vertices.iter() {
        commands.spawn((
            Mesh3d(sphere_mesh.clone()),
            MeshMaterial3d(sphere_material.clone()),
            Transform::from_translation(*pos),
        ));
    }

    stored.needs_dummies = false;
}
