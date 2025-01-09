use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use super::components::*;
use bevy::render::mesh::Indices;
use crate::tools::colors::*;
use super::mouse_part_systems::*;

fn create_mesh_for_face(vertices: &[Vec3]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    
    // Convert quad vertices to two triangles
    let positions = vertices.to_vec();
    let indices = vec![0, 1, 2, 2, 3, 0];
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    
    // Calculate normal for the face
    let normal = calculate_face_normal(vertices);
    let normals: Vec<[f32; 3]> = vec![[normal.x, normal.y, normal.z]; 4];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    
    mesh
}

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

    // Create parent entity with Part component
    let mut part = Part::new();
    part.vertices = vertices.clone();
    part.edges = edges.clone();
    part.faces = faces.clone();

    let parent = commands.spawn((
        Transform::from_xyz(0.0, 0.5, 0.0),
        part,
    )).id();

    // Define face vertices for each face of the cube
    let face_vertices = vec![
        // Front face
        vec![points[0], points[1], points[2], points[3]],
        // Back face
        vec![points[4], points[5], points[6], points[7]],
        // Right face
        vec![points[1], points[2], points[6], points[5]],
        // Left face
        vec![points[0], points[3], points[7], points[4]],
        // Top face
        vec![points[3], points[2], points[6], points[7]],
        // Bottom face
        vec![points[0], points[1], points[5], points[4]],
    ];

    // Spawn each face as a separate entity
    for (i, vertices) in face_vertices.iter().enumerate() {
        let face = faces[i].clone();
        commands.spawn((
            Mesh3d(meshes.add(create_mesh_for_face(vertices))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 0.5, 0.0),
            face,
        ))
        .set_parent(parent)
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(no_change_matl.clone()))
        .observe(update_material_on::<Pointer<Down>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Up>>(hover_matl.clone()))
        .observe(face_selection::<Pointer<Down>>(pressed_matl.clone()));
    }

    // Add rotation to parent entity
    commands.entity(parent)
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
    
    // Define face configurations for all six faces
    let face_configs = vec![
        // Front face (0,1,2,3)
        (vec![0, 1, 2, 3], vec![
            edges[0].start.coordinates,
            edges[1].start.coordinates,
            edges[2].start.coordinates,
            edges[3].start.coordinates,
        ]),
        // Back face (4,5,6,7)
        (vec![4, 5, 6, 7], vec![
            edges[4].start.coordinates,
            edges[5].start.coordinates,
            edges[6].start.coordinates,
            edges[7].start.coordinates,
        ]),
        // Right face (1,2,6,5)
        (vec![1, 2, 10, 9], vec![
            edges[1].start.coordinates,
            edges[2].start.coordinates,
            edges[6].start.coordinates,
            edges[5].start.coordinates,
        ]),
        // Left face (0,3,7,4)
        (vec![0, 3, 11, 8], vec![
            edges[0].start.coordinates,
            edges[3].start.coordinates,
            edges[7].start.coordinates,
            edges[4].start.coordinates,
        ]),
        // Top face (3,2,6,7)
        (vec![2, 3, 11, 10], vec![
            edges[2].start.coordinates,
            edges[3].start.coordinates,
            edges[7].start.coordinates,
            edges[6].start.coordinates,
        ]),
        // Bottom face (0,1,5,4)
        (vec![0, 1, 9, 8], vec![
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
        });
    }

    faces
}

pub fn extrude_faces(
    part: &mut Part,
    extrusion_vector: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent_entity: Entity,
) {
    let mut new_vertices = Vec::new();
    let mut new_edges = Vec::new();
    let mut new_faces = Vec::new();

    // Create new vertices by extruding the existing vertices of the selected faces
    for face in &part.selected_faces {
        let mut face_new_vertices = Vec::new();
        let mut face_vertices_coords = Vec::new();

        // Create new vertices for this face
        for vertex in &face.vertices {
            let new_coord = vertex.coordinates + extrusion_vector;
            let new_vertex = Vertex { coordinates: new_coord };
            face_new_vertices.push(new_vertex.clone());
            face_vertices_coords.push(new_coord);
            
            // Only add to global new_vertices if not already present
            if !new_vertices.iter().any(|v: &Vertex| (v.coordinates - new_coord).length() < 0.0001) {
                new_vertices.push(new_vertex);
            }
        }

        // Create new edges for the extruded face
        let mut new_face_edges = Vec::new();
        for i in 0..face.vertices.len() {
            let next_i = (i + 1) % face.vertices.len();
            
            let new_edge = Edge {
                start: face_new_vertices[i].clone(),
                end: face_new_vertices[next_i].clone(),
            };
            new_edges.push(new_edge.clone());
            new_face_edges.push(new_edge);

            // Create edge connecting original to extruded vertex
            let connecting_edge = Edge {
                start: face.vertices[i].clone(),
                end: face_new_vertices[i].clone(),
            };
            new_edges.push(connecting_edge);
        }

        // Create and spawn the extruded face
        let extruded_face = Face {
            vertices: face_new_vertices.clone(),
            edges: new_face_edges,
            normal: face.normal,
        };
        new_faces.push(extruded_face.clone());

        // Spawn mesh entity for extruded face
        commands.spawn((
            Mesh3d(meshes.add(create_mesh_for_face(&face_vertices_coords))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 0.5, 0.0),
            extruded_face,
        ))
        .set_parent(parent_entity);

        // Create and spawn side faces
        for i in 0..face.vertices.len() {
            let next_i = (i + 1) % face.vertices.len();
            
            let side_vertices = vec![
                face.vertices[i].clone(),
                face.vertices[next_i].clone(),
                face_new_vertices[next_i].clone(),
                face_new_vertices[i].clone(),
            ];

            let side_vertices_coords: Vec<Vec3> = side_vertices.iter()
                .map(|v: &Vertex| v.coordinates)
                .collect();

            let side_edges = vec![
                Edge { start: face.vertices[i].clone(), end: face.vertices[next_i].clone() },
                Edge { start: face.vertices[next_i].clone(), end: face_new_vertices[next_i].clone() },
                Edge { start: face_new_vertices[next_i].clone(), end: face_new_vertices[i].clone() },
                Edge { start: face_new_vertices[i].clone(), end: face.vertices[i].clone() },
            ];

            let side_face = Face {
                vertices: side_vertices,
                edges: side_edges,
                normal: calculate_face_normal(&side_vertices_coords),
            };
            new_faces.push(side_face.clone());

            // Spawn mesh entity for side face
            commands.spawn((
                Mesh3d(meshes.add(create_mesh_for_face(&side_vertices_coords))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(0.0, 0.5, 0.0),
                side_face,
            ))
            .set_parent(parent_entity);
        }
    }

    // Add the new vertices, edges, and faces to the part
    part.vertices.extend(new_vertices);
    part.edges.extend(new_edges);
    part.faces.extend(new_faces);
}
