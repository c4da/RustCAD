use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use super::components::*;
use bevy::render::mesh::Indices;
use crate::tools::colors::*;
use super::mouse_part_systems::*;

pub fn create_3d_object_system(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    points: Vec<Vec3>,
) {

    let no_change_matl = materials.add(NO_CHANGE_COLOR);
    let hover_matl = materials.add(HOVER_COLOR);
    let pressed_matl = materials.add(PRESSED_COLOR);

    let vertices = points_to_vertices(&points);

    let edges = create_edges_from_vertices(&vertices);
    // Spawn the mesh and material as a PbrBundle
    commands.spawn((
        Mesh3d(meshes.add(create_mesh_for_object(points))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Part {
            vertices: vertices.clone(),
            edges: create_edges_from_vertices(&vertices).clone(),
            faces: create_faces_from_edges(&edges).clone(),
        },
        )   
    )
    .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    .observe(update_material_on::<Pointer<Out>>(no_change_matl.clone()))
    .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
    .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
    .observe(rotate_on_drag);
}

fn create_mesh_for_object(points: Vec<Vec3>) -> Mesh {
    // Create a mesh for the 3D object
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points.clone());
    mesh.insert_indices(Indices::U32(vec![
        0, 1, 2, 2, 3, 0, // Bottom face
        4, 5, 6, 6, 7, 4, // Top face
        0, 1, 5, 5, 4, 0, // Side faces
        1, 2, 6, 6, 5, 1,
        2, 3, 7, 7, 6, 2,
        3, 0, 4, 4, 7, 3,
    ]));
    return mesh;
}

fn points_to_vertices(points: &Vec<Vec3>) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for point in points.iter() {
        vertices.push(point_to_vertex(point.clone()));
    }
    vertices
}

fn point_to_vertex(point: Vec3) -> Vertex {
    Vertex { coordinates: point }
}

// Algorithm to create edges from vertices
fn create_edges_from_vertices(vertices: &[Vertex]) -> Vec<Edge> {
    let mut edges = Vec::new();

    // Define the connections between vertices (cube example)
    let connections = vec![
        (0, 1), (1, 2), (2, 3), (3, 0), // Bottom face
        (4, 5), (5, 6), (6, 7), (7, 4), // Top face
        (0, 4), (1, 5), (2, 6), (3, 7), // Side edges
    ];

    for (start_idx, end_idx) in connections {
        edges.push(Edge {
            start: vertices[start_idx].clone(), 
            end: vertices[end_idx].clone(),
        });
    }

    edges
}

// Algorithm to create edges from vertices
fn create_faces_from_edges(edges: &Vec<Edge>) -> Vec<Face> {

    let faces = vec![
        Face { edges: vec![edges[0].clone(), edges[1].clone(), edges[2].clone(), edges[3].clone()] },
        Face { edges: vec![edges[4].clone(), edges[5].clone(), edges[6].clone(), edges[7].clone()] },
        Face { edges: vec![edges[8].clone(), edges[9].clone(), edges[10].clone(), edges[11].clone()] },
    ];

    faces
}