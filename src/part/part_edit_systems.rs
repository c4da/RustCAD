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

    let vertices = Vertex::points_to_vertices(&points);
    let edges = create_edges_from_vertices(&vertices);
    let faces = create_faces_from_edges(&edges);

    // Spawn the mesh and material as a PbrBundle
    let mut part = Part::new();
    part.vertices = vertices.clone();
    part.edges = edges.clone();
    part.faces = faces.clone();

    commands.spawn((
        Mesh3d(meshes.add(create_mesh_for_object(points))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
        part,
    ))
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
fn calculate_face_normal(vertices: &[Vec3]) -> Vec3 {
    if vertices.len() < 3 {
        return Vec3::Z; // Default normal if not enough vertices
    }
    let v1 = vertices[1] - vertices[0];
    let v2 = vertices[2] - vertices[0];
    v1.cross(v2).normalize()
}

fn create_faces_from_edges(edges: &Vec<Edge>) -> Vec<Face> {
    let mut faces = Vec::new();
    
    // Define face configurations
    let face_configs = vec![
        // Bottom face
        (vec![0, 1, 2, 3], vec![
            edges[0].start.coordinates,
            edges[1].start.coordinates,
            edges[2].start.coordinates,
            edges[3].start.coordinates,
        ]),
        // Top face
        (vec![4, 5, 6, 7], vec![
            edges[4].start.coordinates,
            edges[5].start.coordinates,
            edges[6].start.coordinates,
            edges[7].start.coordinates,
        ]),
        // Front face
        (vec![8, 9, 10, 11], vec![
            edges[0].start.coordinates,
            edges[1].start.coordinates,
            edges[5].start.coordinates,
            edges[4].start.coordinates,
        ]),
    ];
    
    for (edge_indices, points) in face_configs {
        let mut vertices: Vec<Vertex> = Vec::new();
        points.iter().for_each(|&point| {
            vertices.push(Vertex { coordinates: point });
        });
        
        let face_edges = edge_indices.iter()
            .map(|&i| edges[i].clone())
            .collect();
        
        faces.push(Face {
            vertices: vertices.clone(),
            edges: face_edges,
            normal: calculate_face_normal(&points),
            part: Part::new(),
        });
    }

    faces
}


fn extrude_faces(part: &mut Part, extrusion_vector: Vec3) {
    let mut new_vertices = Vec::new();
    let mut new_edges = Vec::new();
    let mut new_faces = Vec::new();

    // Create new vertices by extruding the existing vertices of the selected faces
    for face in &part.faces {
        if part.selected_faces.contains(face) {
            for vertex in &face.get_vertices() {
                let new_vertex = Vertex {
                    coordinates: vertex.coordinates + extrusion_vector,
                };
                new_vertices.push(new_vertex);
            }
        }
    }

    // Create new edges by connecting the new vertices
    for face in &part.faces {
        if part.selected_faces.contains(&face) {
            for edge in &face.edges {
                let start_idx = part.vertices.iter().position(|v| v == &edge.start).unwrap();
                let end_idx = part.vertices.iter().position(|v| v == &edge.end).unwrap();
                let new_start = new_vertices[start_idx];
                let new_end = new_vertices[end_idx];
                new_edges.push(Edge {
                    start: new_start,
                    end: new_end,
                });
            }
        }
    }

    // Create new faces by connecting the new edges
    for face in &part.faces {
        if part.selected_faces.contains(&face) {
            let mut vertices: Vec<Vertex> = Vec::new();
            let mut new_face_edges = Vec::new();
            for edge in &face.edges {
                let start_idx = part.vertices.iter().position(|v| v == &edge.start).unwrap();
                let end_idx = part.vertices.iter().position(|v| v == &edge.end).unwrap();
                let new_start = new_vertices[start_idx];
                let new_end = new_vertices[end_idx];
                new_face_edges.push(Edge {
                    start: new_start,
                    end: new_end,
                });
            }
            new_face_edges.iter().for_each(|edge| vertices.extend(edge.get_vertices()));
            new_faces.push(Face {
                vertices: vertices,
                edges: new_face_edges,
                normal: face.normal,
                part: part.clone(),
            });
        }
    }

    // Add the new vertices, edges, and faces to the part
    part.vertices.extend(new_vertices);
    part.edges.extend(new_edges);
    part.faces.extend(new_faces);
}



// pub fn extrude_faces(
//     mut query: Query<&mut Part>,
//     extrusion: Res<ExtrusionParams>,
// ) {
//     for mut part in query.iter_mut() {
//         for &face_idx in &extrusion.selected_faces {
//             if let Some(face) = part.faces.get(face_idx).cloned() {
//                 // Get vertices from the face edges
//                 let face_vertices: Vec<Vec3> = face.edges.iter()
//                     .map(|edge| edge.start.coordinates)
//                     .collect();

//                 // Create new vertices by extruding existing ones
//                 let new_vertices: Vec<Vertex> = face_vertices.iter()
//                     .map(|&v| Vertex {
//                         coordinates: v + face.normal * extrusion.distance
//                     })
//                     .collect();

//                 // Add new vertices to the part
//                 let start_idx = part.vertices.len();
//                 part.vertices.extend(new_vertices.clone());

//                 // Create new edges for the extruded face
//                 let mut new_edges = Vec::new();
//                 for i in 0..face_vertices.len() {
//                     let next_i = (i + 1) % face_vertices.len();
                    
//                     // Edge connecting new vertices
//                     new_edges.push(Edge {
//                         start: new_vertices[i].clone(),
//                         end: new_vertices[next_i].clone(),
//                     });
                    
//                     // Vertical edge connecting original to extruded vertex
//                     new_edges.push(Edge {
//                         start: Vertex { coordinates: face_vertices[i] },
//                         end: new_vertices[i].clone(),
//                     });
//                 }

//                 // Add new edges
//                 let edge_start_idx = part.edges.len();
//                 part.edges.extend(new_edges);

//                 // Create the extruded face
//                 let extruded_face_edges: Vec<Edge> = (0..face_vertices.len())
//                     .map(|i| part.edges[edge_start_idx + i * 2].clone())
//                     .collect();

//                 let part_clone = part.clone();
//                 part.faces.push(Face {
//                     edges: extruded_face_edges,
//                     normal: face.normal,
//                     part: part_clone,
//                 });

//                 // Create side faces
//                 for i in 0..face_vertices.len() {
//                     let next_i = (i + 1) % face_vertices.len();
//                     let side_vertices = vec![
//                         face_vertices[i],
//                         face_vertices[next_i],
//                         new_vertices[next_i].coordinates,
//                         new_vertices[i].coordinates,
//                     ];

//                     let side_normal = calculate_face_normal(&side_vertices);
//                     let side_edges = vec![
//                         Edge {
//                             start: Vertex { coordinates: face_vertices[i] },
//                             end: Vertex { coordinates: face_vertices[next_i] },
//                         },
//                         part.edges[edge_start_idx + next_i * 2 + 1].clone(),
//                         Edge {
//                             start: new_vertices[next_i].clone(),
//                             end: new_vertices[i].clone(),
//                         },
//                         part.edges[edge_start_idx + i * 2 + 1].clone(),
//                     ];

//                     let part_clone = part.clone();
//                     part.faces.push(Face {
//                         edges: side_edges,
//                         normal: side_normal,
//                         part: part_clone,
//                     });
//                 }
//             }
//         }
//     }
// }
