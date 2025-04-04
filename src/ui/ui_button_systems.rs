// use crate::tools::tools::{transform_mouse_pointer_to_vect};

use bevy::prelude::*;
use bevy::utils::warn;

use super::ui_elements::*;
use crate::part::components::*;

use crate::part;
use crate::tools::colors::{PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, RED};
use crate::ui::components::*;

pub fn button_highlight_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            Option<&ToggleableButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (interaction, mut color, mut border_color, children, toggleable) in &mut interaction_query {
        // Skip color management for toggleable buttons
        if toggleable.is_some() {
            continue;
        }

        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // **text = "Press".to_string();
                *color = PRESSED_BUTTON_COLOR.into();
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
                *color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // **text = "Button".to_string();
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

// Add resource to track current mode
#[derive(Resource, PartialEq)]
pub enum EditorMode {
    SelectFace,
    SelectEdge,
    MoveFace,
    RotatePart
}

impl Default for EditorMode {
    fn default() -> Self {
        EditorMode::SelectFace  // Default to select mode
    }
}
// System handles buttons behaviour, not action that the button should trigger
pub fn button_action_system(
    mut interaction_query: Query<
        (&Interaction, &ToolbarButtonType, Option<&mut ToggleableButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut button_events: EventWriter<ToolbarAction>,
    mut mode: ResMut<EditorMode>,
    mut part_query: Query<&mut Part>,
) {
    for (interaction, button_type, toggleable) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                ToolbarButtonType::SelectEdgeMode |
                ToolbarButtonType::SelectFaceMode |
                ToolbarButtonType::RotatePart |
                ToolbarButtonType::MoveFace => {
                    if let Some(mut _toggle) = toggleable {
                        match button_type {
                            ToolbarButtonType::SelectEdgeMode => {
                                *mode = EditorMode::SelectEdge;
                                // Clear all selections when switching modes
                                for mut part in part_query.iter_mut() {
                                    part.selected_faces.clear();
                                    part.selected_edges.clear();
                                    part.selected_vertices.clear();
                                }
                                button_events.send(ToolbarAction::SelectEdgeMode);
                            }
                            ToolbarButtonType::SelectFaceMode => {
                                *mode = EditorMode::SelectFace;
                                // Clear all selections when switching modes
                                for mut part in part_query.iter_mut() {
                                    part.selected_faces.clear();
                                    part.selected_edges.clear();
                                    part.selected_vertices.clear();
                                }
                                button_events.send(ToolbarAction::SelectFaceMode);
                            }
                            ToolbarButtonType::RotatePart => {
                                *mode = EditorMode::RotatePart;
                                // Clear all selections when switching modes
                                for mut part in part_query.iter_mut() {
                                    part.selected_faces.clear();
                                    part.selected_edges.clear();
                                    part.selected_vertices.clear();
                                }
                                button_events.send(ToolbarAction::RotatePart);
                            }
                            ToolbarButtonType::MoveFace => {
                                *mode = EditorMode::MoveFace;
                                // Clear all selections when switching modes
                                for mut part in part_query.iter_mut() {
                                    part.selected_faces.clear();
                                    part.selected_edges.clear();
                                    part.selected_vertices.clear();
                                }
                                button_events.send(ToolbarAction::MoveFace);
                            }
                            _ => {}
                        }
                    }
                }
                ToolbarButtonType::Extrude => {
                    println!("Extrude button pressed"); // Debug print
                    button_events.send(ToolbarAction::Extrude);
                }
                ToolbarButtonType::CreateVertex => {
                    button_events.send(ToolbarAction::CreateVertex);
                }
                ToolbarButtonType::CreateEdge => {
                    button_events.send(ToolbarAction::CreateEdge);
                }
                ToolbarButtonType::CreateFace => {
                    button_events.send(ToolbarAction::CreateFace);
                }
                ToolbarButtonType::Delete => {
                    button_events.send(ToolbarAction::Delete);
                }
            }
        }
    }
}

// System to handle toolbar actions - actual actions that the button should trigger
pub fn handle_toolbar_actions(
    mut commands: Commands,
    mut events: EventReader<ToolbarAction>,
    mut part_query: Query<(Entity, &mut Part)>,
    extrusion_params: ResMut<ExtrusionParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        match event {
            ToolbarAction::Extrude => {
                let mut params = extrusion_params.clone();
                
                // for (_point, normal) in pointers
                //                         .iter()
                //                         .filter_map(|interaction| interaction.get_nearest_hit())
                //                         .filter_map(|(_entity, hit)| hit.position.zip(hit.normal)) {
                //     println!("Point: {:?}, Normal: {:?}", _point, normal);
                //     params.direction = normal.normalize();
                // };

                // test extrusion parameters
                params.direction = Vec3::Y;
                params.distance = 1.0;

                for (entity, mut part) in part_query.iter_mut() {
                    println!("Selected face: {:?}", part.selected_faces);
                    if part.selected_faces.is_empty() {
                        warn(Result::Err("No faces selected for extrusion"));
                        continue;
                    }
                    // Handle extrusion
                    // let extrusion_vector = params.direction * params.distance;
                    part::extrude_faces(&mut part, &params, &mut commands, &mut meshes, &mut materials, entity);
                    part.selected_faces.clear();
                }
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
            ToolbarAction::SelectFaceMode => {
                // Handle face selection mode
            }
            ToolbarAction::SelectEdgeMode => {
                // Handle edge selection mode
            }
            ToolbarAction::RotatePart => {},
            ToolbarAction::MoveFace => {},
        }
    }
}

// Add system to update button visuals based on state
pub fn update_selection_mode_buttons(
    mut buttons: Query<
        (
            &ToolbarButtonType,
            &mut ToggleableButton,
            &mut BackgroundColor,
            &Interaction,
        ),
        With<Button>,
    >,
    mode: Res<EditorMode>,
) {
    for (button_type, mut toggleable, mut color, interaction) in buttons.iter_mut() {
        match button_type {
            ToolbarButtonType::SelectFaceMode => {
                toggleable.is_active = matches!(*mode, EditorMode::SelectFace);
                *color = match (*interaction, toggleable.is_active) {
                    (Interaction::Hovered, false) => HOVERED_BUTTON_COLOR.into(),
                    (_, true) => PRESSED_BUTTON_COLOR.into(),
                    _ => NORMAL_BUTTON_COLOR.into(),
                };
            }
            ToolbarButtonType::SelectEdgeMode => {
                toggleable.is_active = matches!(*mode, EditorMode::SelectEdge);
                *color = match (*interaction, toggleable.is_active) {
                    (Interaction::Hovered, false) => HOVERED_BUTTON_COLOR.into(),
                    (_, true) => PRESSED_BUTTON_COLOR.into(),
                    _ => NORMAL_BUTTON_COLOR.into(),
                };
            }
            ToolbarButtonType::RotatePart => {
                toggleable.is_active = matches!(*mode, EditorMode::RotatePart);
                *color = match (*interaction, toggleable.is_active) {
                    (Interaction::Hovered, false) => HOVERED_BUTTON_COLOR.into(),
                    (_, true) => PRESSED_BUTTON_COLOR.into(),
                    _ => NORMAL_BUTTON_COLOR.into(),
                };
            }
            ToolbarButtonType::MoveFace => {
                toggleable.is_active = matches!(*mode, EditorMode::MoveFace);
                *color = match (*interaction, toggleable.is_active) {
                    (Interaction::Hovered, false) => HOVERED_BUTTON_COLOR.into(),
                    (_, true) => PRESSED_BUTTON_COLOR.into(),
                    _ => NORMAL_BUTTON_COLOR.into(),
                };
            }
            _ => {}
        }
    }
}
