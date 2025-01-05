use bevy::prelude::*;

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub coordinates: Vec3,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            coordinates: Vec3::new(x, y, z),
        }
    }

    pub fn get_coordinates(&self) -> Vec3 {
        self.coordinates
    }

    pub fn point_to_vertex(point: Vec3) -> Vertex {
        Vertex { coordinates: point }
    }

    pub fn points_to_vertices(points: &Vec<Vec3>) -> Vec<Vertex> {
        points.into_iter().map(|p| Vertex::point_to_vertex(*p)).collect()
    }
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
}

impl Edge {
    pub fn new(start: Vertex, end: Vertex) -> Self {
        Edge { start, end }
    }

    pub fn with_points(points: Vec<Vec3>) -> Self {
        Edge {
            start: Vertex::point_to_vertex(points[0]),
            end: Vertex::point_to_vertex(points[1]),
        }
    }

    pub fn with_vertices(vertices: Vec<Vertex>) -> Self {
        Edge {
            start: vertices[0],
            end: vertices[1],
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        vec![self.start, self.end]
    }
    
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Face {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub normal: Vec3,
    pub part: Part,
}

impl Face {
    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }
}

#[derive(Component, Debug, Resource)]
pub struct ExtrusionParams {
    pub direction: Vec3,
    pub distance: f32,
    pub selected_faces: Vec<usize>,
}

/// A marker component
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Part {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub selected_vertices: Vec<Vertex>,
    pub selected_edges: Vec<Edge>,
    pub selected_faces: Vec<Face>,
}

impl Part {
    pub fn new() -> Self {
        Part {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            selected_vertices: Vec::new(),
            selected_edges: Vec::new(),
            selected_faces: Vec::new(),
        }
    }

    pub fn with_points(points: Vec<Vec3>) -> Self {
        let mut part = Part {
            vertices: Vertex::points_to_vertices(&points),
            edges: Vec::new(),
            faces: Vec::new(),
            selected_vertices: Vec::new(),
            selected_edges: Vec::new(),
            selected_faces: Vec::new(),
        };
        part.assign_faces_to_part();
        part
    }

    fn assign_faces_to_part(&mut self) {
        let cloned_part: Part = self.clone();
        for face in &mut self.faces {
            face.part = cloned_part.clone();
        }
    }
}
