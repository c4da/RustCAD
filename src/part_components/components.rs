use bevy::prelude::*;

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component, Clone)]
pub struct Vertex{
    pub coordinates: Vec3,
}

#[derive(Component, Clone)]
pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
}

#[derive(Component, Clone)]
pub struct Face {
    pub edges: Vec<Edge>,
}

/// A marker component
#[derive(Component)]
pub struct Part {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}
