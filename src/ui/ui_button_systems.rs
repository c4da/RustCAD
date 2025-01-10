// use crate::tools::tools::{transform_mouse_pointer_to_vect};

use bevy::{picking::pointer::PointerInteraction, prelude::*};
use bevy::utils::warn;

use super::ui_elements::*;
use crate::part::components::*;

use crate::part;
use crate::tools::colors::{GRAY, PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, RED};

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
    mut commands: Commands,
    mut events: EventReader<ToolbarAction>,
    mut part_query: Query<(Entity, &mut Part)>,
    mut extrusion_params: ResMut<ExtrusionParams>,
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
        }
    }
}
