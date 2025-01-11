use bevy::prelude::*;
// Button types for different CAD operations
#[derive(Component)]
pub enum ToolbarButtonType {
    Extrude,
    CreateVertex,
    CreateEdge,
    CreateFace,
    Delete,
    SelectFaceMode,
    SelectEdgeMode,
}

#[derive(Component,)]
pub struct ToolbarButton;

//Component to track button state
#[derive(Component)]
pub struct ToggleableButton {
    pub is_active: bool,
}
