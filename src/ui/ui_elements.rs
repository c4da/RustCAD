use bevy::prelude::*;
use bevy::utils::warn;
use crate::part::{self, primitives};
use crate::tools::colors::*;
use super::components::*;

//UI Constants
const BUTTON_HEIGHT: f32 = 28.0;
const BUTTON_MARGIN: f32 = 2.0;
const TEXT_SIZE: f32 = 14.0;
const HEADER_TEXT_SIZE: f32 = 13.0;
const PANEL_PADDING: f32 = 5.0;
const SECTION_SPACING: f32 = 10.0;

// Panel dimensions
const TOP_BAR_HEIGHT: f32 = 32.0;
const LEFT_TOOLBAR_WIDTH: f32 = 160.0;
const RIGHT_PANEL_WIDTH: f32 = 240.0;

#[derive(Bundle)]
pub struct CustomTextBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
}

impl CustomTextBundle {
    pub fn new(label: &str, size: f32) -> Self {
        Self {
            text: Text::new(label),
            font: TextFont {
                font_size: size,
                ..default()
            },
            color: TextColor(TEXT_COLOR),
        }
    }
}

pub fn setup_ui(mut commands: Commands) {
    // Create a viewport entity that will not block picking
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        PickingBehavior::IGNORE,
    ))
    .with_children(|parent| {
        // Top bar
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(TOP_BAR_HEIGHT),
                padding: UiRect::horizontal(Val::Px(PANEL_PADDING)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(HEADER_BG),
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            setup_top_toolbar(parent);
        });

        // Left toolbar
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                width: Val::Px(LEFT_TOOLBAR_WIDTH),
                top: Val::Px(TOP_BAR_HEIGHT),
                bottom: Val::Px(0.0),
                padding: UiRect::all(Val::Px(PANEL_PADDING)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(BG_COLOR),
            BorderColor(NEAR_BLACK),
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            setup_side_toolbar(parent);
        });

        // Right panel
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                width: Val::Px(RIGHT_PANEL_WIDTH),
                top: Val::Px(TOP_BAR_HEIGHT),
                bottom: Val::Px(0.0),
                padding: UiRect::all(Val::Px(PANEL_PADDING)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(BG_COLOR),
            BorderColor(NEAR_BLACK),
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            setup_properties_panel(parent);
        });
    });
}

fn setup_top_toolbar(parent: &mut ChildBuilder) {
    let menu_items = ["File", "Edit", "View", "Window", "Help"];
    
    for item in menu_items {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(TOP_BAR_HEIGHT),
                margin: UiRect::right(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON_COLOR),
            BorderColor(NEAR_BLACK),
            Interaction::None,
            ToolbarButton,
        ))
        .with_children(|parent| {
            parent.spawn(CustomTextBundle::new(item, HEADER_TEXT_SIZE));
        });
    }
}

fn setup_properties_panel(parent: &mut ChildBuilder) {
    // Properties panel header
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(BUTTON_HEIGHT),
            padding: UiRect::horizontal(Val::Px(8.0)),
            margin: UiRect::bottom(Val::Px(PANEL_PADDING)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(HEADER_BG),
        BorderColor(NEAR_BLACK),
    ))
    .with_children(|parent| {
        parent.spawn(CustomTextBundle::new("Properties", HEADER_TEXT_SIZE));
    });
}

fn setup_side_toolbar(parent: &mut ChildBuilder) {
    let sections = [
        ("Create", vec![
            ("Vertex", ToolbarButtonType::CreateVertex),
            ("Edge", ToolbarButtonType::CreateEdge),
            ("Face", ToolbarButtonType::CreateFace),
        ]),
        ("Edit", vec![
            ("Extrude", ToolbarButtonType::Extrude),
            ("Delete", ToolbarButtonType::Delete),
        ]),
        ("Select", vec![
            ("Face", ToolbarButtonType::SelectFaceMode),
            ("Edge", ToolbarButtonType::SelectEdgeMode),
        ]),
        ("Transform", vec![
            ("Move", ToolbarButtonType::MoveFace),
            ("Rotate", ToolbarButtonType::RotatePart),
        ]),
    ];

    // Toolbar scroll container
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            height: Val::Auto,
            width: Val::Percent(100.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        for (section_name, tools) in sections {
            spawn_tool_section(parent, section_name, &tools);
        }
    });
}

fn spawn_tool_section(parent: &mut ChildBuilder, title: &str, tools: &[(&str, ToolbarButtonType)]) {
    // Section container
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(SECTION_SPACING)),
            width: Val::Percent(100.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        // Section header
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(BUTTON_HEIGHT),
                padding: UiRect::horizontal(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Px(PANEL_PADDING)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(HEADER_BG),
            BorderColor(NEAR_BLACK),
        ))
        .with_children(|parent| {
            parent.spawn(CustomTextBundle::new(title, HEADER_TEXT_SIZE));
        });

        // Tools
        for (label, button_type) in tools {
            spawn_tool_button(parent, label, *button_type);
        }
    });
}

fn spawn_tool_button(parent: &mut ChildBuilder, label: &str, button_type: ToolbarButtonType) {
    let is_toggleable = matches!(
        button_type,
        ToolbarButtonType::SelectFaceMode
            | ToolbarButtonType::SelectEdgeMode
            | ToolbarButtonType::RotatePart
            | ToolbarButtonType::MoveFace
    );

    let mut button = parent.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(BUTTON_HEIGHT),
            margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
            padding: UiRect::horizontal(Val::Px(8.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON_COLOR),
        BorderColor(NEAR_BLACK),
        Interaction::None,
        ToolbarButton,
        button_type,
    ));

    if is_toggleable {
        button.insert(ToggleableButton { is_active: false });
    }

    button.with_children(|parent| {
        parent.spawn(CustomTextBundle::new(label, TEXT_SIZE));
    });
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

pub fn add_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    warn(Result::Err("Adding box"));

    let points = primitives::CubePoints::get_points();

    part::create_3d_object_system(commands, meshes, materials, points);
}
