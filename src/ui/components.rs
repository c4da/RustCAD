use bevy::prelude::*;
// Button types for different CAD operations
#[derive(Component, Copy, Clone)]
pub enum ToolbarButtonType {
    Extrude,
    CreateVertex,
    CreateEdge,
    CreateFace,
    Delete,
    SelectFaceMode,
    SelectEdgeMode,
    RotatePart,
    MoveFace,
}

#[derive(Component,)]
pub struct ToolbarButton;

//Component to track button state
#[derive(Component)]
pub struct ToggleableButton {
    pub is_active: bool,
}
