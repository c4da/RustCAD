use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use crate::part_components::components::*;
use bevy::render::mesh::Indices;

pub fn create_3d_object_system(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mut points: Vec<Vec3>,
) {

    let mut vertices = Vec::new();
    // Define the vertices of the 3D object
    for point in points.iter() {
        vertices.push(Vertex { coordinates: point.clone() });
    }

    // Create edges between the vertices
    let edges = vec![
        (0, 1), (1, 2), (2, 3), (3, 0), // Bottom face
        (4, 5), (5, 6), (6, 7), (7, 4), // Top face
        (0, 4), (1, 5), (2, 6), (3, 7), // Side edges
    ];


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

    // Create a material for the 3D object
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    let edges = create_edges_from_vertices(&vertices);
    // Spawn the mesh and material as a PbrBundle
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Part {
            vertices: vertices.clone(),
            edges: create_edges_from_vertices(&vertices).clone(),
            faces: create_faces_from_edges(&edges).clone(),
        },
        )   
    );
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