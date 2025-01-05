use bevy::prelude::*;
use bevy::utils::warn;
use bevy::ui::widget::*;
use crate::part::components::*; // Import the Vertex component

use crate::part;
use crate::tools::colors::{GRAY, PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, RED};
use crate::tools::tools::{create_vertex_dummies, get_vertices};

// UI Constants
const BUTTON_WIDTH: f32 = 120.0;
const BUTTON_HEIGHT: f32 = 30.0;
const BUTTON_MARGIN: f32 = 5.0;
const TEXT_SIZE: f32 = 16.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TOOLBAR_WIDTH: f32 = 150.0;
const TOOLBAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);

// Button types for different CAD operations
#[derive(Component)]
pub enum ToolbarButtonType {
    Extrude,
    CreateVertex,
    CreateEdge,
    CreateFace,
    Delete,
}

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

#[derive(Component,)]
pub struct ToolbarButton;

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
}

pub fn button_highlight_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (interaction, 
        mut color, 
        mut border_color, children) in &mut interaction_query {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                //Result::Err("xx")
                warn(Result::Err(_text));

                let mut _text = text_query.get_mut(children[0]).unwrap();
                //**_text.contains("Box")
                if _text.contains("Box") {
                    add_box(&mut commands, &mut meshes, &mut materials);
                }
            }
            Interaction::Hovered => {
                // **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn add_box(
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
}

pub fn button_action_system(
    mut interaction_query: Query<
        (&Interaction, &ToolbarButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut button_events: EventWriter<ToolbarAction>,
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let action = match button_type {
                ToolbarButtonType::Extrude => ToolbarAction::Extrude,
                ToolbarButtonType::CreateVertex => ToolbarAction::CreateVertex,
                ToolbarButtonType::CreateEdge => ToolbarAction::CreateEdge,
                ToolbarButtonType::CreateFace => ToolbarAction::CreateFace,
                ToolbarButtonType::Delete => ToolbarAction::Delete,
            };
            button_events.send(action);
        }
    }
}

pub fn handle_toolbar_actions(
    mut events: EventReader<ToolbarAction>,
    mut query: Query<&mut Part>,
    mut extrusion_params: ResMut<ExtrusionParams>,
) {
    for event in events.read() {
        match event {
            ToolbarAction::Extrude => {
                let mut params = extrusion_params.clone();
                // test extrusion parameters
                params.direction = Vec3::Y;
                params.distance = 1.0;
            },
            ToolbarAction::CreateVertex => {
                // Handle vertex creation
            }
            ToolbarAction::CreateEdge => {
                // Handle edge creation
            }
            ToolbarAction::CreateFace => {
                // Handle face creation
            }
            ToolbarAction::Delete => {
                // Handle deletion
            }
        }
    }
}

// #[derive(Event)]
// pub enum ButtonAction {
//     Save,
//     Load,
//     Exit,
//     ViewVertices,
// }

// // Component to identify button type
// #[derive(Component)]
// pub enum ButtonType {
//     Save,
//     Load,
//     Exit,
//     ViewVertices,
// }

// pub fn button_action_system(
//     mut interaction_query: Query<
//         (&Interaction, &mut BackgroundColor, &ButtonType),
//         Changed<Interaction>
//     >,
//     mut button_events: EventWriter<ButtonAction>,
// ) {
//     for (interaction, mut _color, button_type) in &mut interaction_query {
//         if *interaction == Interaction::Pressed {
//             // Emit the appropriate event based on button type
//             match *button_type {
//                 ButtonType::Save => button_events.send(ButtonAction::Save),
//                 ButtonType::Load => button_events.send(ButtonAction::Load),
//                 ButtonType::Exit => button_events.send(ButtonAction::Exit),
//                 ButtonType::ViewVertices => button_events.send(ButtonAction::ViewVertices),
//             };
//         }
//     }
// }

// Add event handler system
// fn handle_button_actions(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut events: EventReader<ButtonAction>,
//     selected: Res<SelectedMesh>,
//     mesh_assests: Res<Assets<Mesh>>,
// ) {
//     for event in events.read() {
//         match event {
//             // ButtonAction::Save => { /* Save logic */ },
//             // ButtonAction::Load => { /* Load logic */ },
//             // ButtonAction::Exit => { /* Exit logic */ },
//             ButtonAction::ViewVertices => { 
//                 println!("View Vertices button pressed");
//                 if let Some(mesh_handle) = &selected.mesh {
//                     if let Some(mesh) = mesh_assests.get(mesh_handle) {
//                         let vertices = get_vertices(mesh);
//                         // create_vertex_dummies(commands, materials, meshes, &vertices);
//                     }
//                 }
//              },
//         }
//     }
// }
