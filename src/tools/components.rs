use bevy::prelude::*;

/// A marker component for our shapes so we can query them separately from the ground plane.
#[derive(Component)]
pub struct Shape;
