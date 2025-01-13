use bevy::prelude::*;
use bevy::utils::warn;
use crate::part::components::*; // Import the Vertex component

use crate::part;
use crate::tools::colors::{GRAY, PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, RED};
use crate::tools::mesh_tools::{get_vertices, create_vertex_dummies};
use super::components::*;

// UI Constants
const BUTTON_WIDTH: f32 = 120.0;
const BUTTON_HEIGHT: f32 = 30.0;
const BUTTON_MARGIN: f32 = 5.0;
const TEXT_SIZE: f32 = 16.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TOOLBAR_WIDTH: f32 = 150.0;
const TOOLBAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);

fn create_button_bundle() -> (Button, Node, BackgroundColor, BorderColor) {
    (
        Button,
        Node {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
            margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(GRAY),
        BorderColor(Color::BLACK),
    )
}

fn create_text_bundle(label: &str) -> (Text, TextFont, TextColor) {
    (
        Text::new(label),
        TextFont {
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
    )
}

fn create_shape_bundle(label: &str) -> (Text, TextFont, TextColor) {
    (
        Text::new(label),
        TextFont {
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
    )
}

pub fn setup_ui(mut commands: Commands) {
    // Top toolbar (keeping existing one)
    commands
        .spawn(
                Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                margin: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Percent(10.),
                        top: Val::Px(0.0),
                        bottom: Val::Percent(15.)
                        },
                // padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Row,
                ..Default::default()
            })
        .with_children(|parent| {
            setup_top_toolbar(parent);
        });

    // Side toolbar (shapes)
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        width: Val::Px(80.0),
        height: Val::Percent(100.0),
        right: Val::Px(0.0),
        margin: UiRect { left: Val::Px(0.0), right: Val::Px(10.0), top: Val::Px(0.0), bottom: Val::Px(0.0) },
        flex_direction: FlexDirection::Column,
        border: UiRect { left: Val::Px(1.0), right: Val::Px(1.0), top: Val::Px(1.0), bottom: Val::Px(1.0) },
        ..Default::default()
    })
        .with_children(|parent| {
            setup_shapes(parent);
    });

    // Side toolbar (new)
    commands
    .spawn(
        Node {
        position_type: PositionType::Absolute,
        width: Val::Px(TOOLBAR_WIDTH),
        height: Val::Percent(100.0),
        margin: UiRect {
                left: Val::Px(0.0),
                right: Val::Percent(10.),
                top: Val::Px(40.0),
                ..Default::default()
                },
        // padding: UiRect::all(Val::Px(5.0)),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.0)),
        ..Default::default()
    }).with_children(|parent| {
            setup_side_toolbar(parent);
        });

}

fn setup_shapes(parent: &mut ChildBuilder) {
    parent
        .spawn((create_button_bundle(), ToolbarButton))
        .with_child(create_shape_bundle("Box"));
}


fn setup_top_toolbar(parent: &mut ChildBuilder) {
    parent
        .spawn((create_button_bundle(), ToolbarButton))
        .with_child(create_text_bundle("File"));
    
    parent
        .spawn((create_button_bundle(), ToolbarButton))
        .with_child(create_text_bundle("Edit"));

    parent
        .spawn((create_button_bundle(), ToolbarButton))
        .with_child(create_text_bundle("View"));
    parent
        .spawn((
            create_button_bundle(),
            ToolbarButton,
            ToolbarButtonType::SelectFaceMode,
            ToggleableButton { is_active: true },  // Select mode on by default
        ))
        .with_child(create_text_bundle("Select Face"));

    parent
        .spawn((
            create_button_bundle(),
            ToolbarButton,
            ToolbarButtonType::SelectEdgeMode,
            ToggleableButton { is_active: false },
        ))
        .with_child(create_text_bundle("Select Edge"));

        parent
        .spawn((
            create_button_bundle(),
            ToolbarButton,
            ToolbarButtonType::RotatePart,
            ToggleableButton { is_active: false },
        ))
        .with_child(create_text_bundle("Rotate Part"));
        parent
            .spawn((
                create_button_bundle(),
                ToolbarButton,
                ToolbarButtonType::MoveFace,
                ToggleableButton { is_active: false },
            ))
            .with_child(create_text_bundle("Move Face"));
}


pub fn add_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,) {
    warn(Result::Err("Adding box"));


    let points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(2.0, 2.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 0.0, 2.0),
        Vec3::new(2.0, 0.0, 2.0),
        Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(0.0, 2.0, 2.0),
    ];

    part::create_3d_object_system(commands, meshes, materials, points);
}

fn setup_side_toolbar(parent: &mut ChildBuilder) {
    // CAD selection buttons
    // spawn_tool_button(parent, "Extrude", ToolbarButtonType::EdgeSelection);
    // spawn_tool_button(parent, "Extrude", ToolbarButtonType::FaceSelection);
    // spawn_tool_button(parent, "Extrude", ToolbarButtonType::PartSelection);
    // CAD Operation buttons
    spawn_tool_button(parent, "Extrude", ToolbarButtonType::Extrude);
    spawn_tool_button(parent, "Add Vertex", ToolbarButtonType::CreateVertex);
    spawn_tool_button(parent, "Add Edge", ToolbarButtonType::CreateEdge);
    spawn_tool_button(parent, "Create Face", ToolbarButtonType::CreateFace);
    spawn_tool_button(parent, "Delete", ToolbarButtonType::Delete);
}

fn spawn_tool_button(parent: &mut ChildBuilder, label: &str, button_type: ToolbarButtonType) {
    parent
        .spawn((
            create_button_bundle(),
            ToolbarButton,
            button_type,
        ))
        .with_child(create_text_bundle(label));
}

#[derive(Event)]
pub enum ToolbarAction {
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