use bevy::picking::mesh_picking::ray_cast::ray_mesh_intersection;
use bevy::prelude::*;
use bevy::utils::warn;
use crate::tools::colors::{GRAY, PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, RED};
use crate::tools::tools::{create_vertex_dummies, get_vertices};

const BUTTON_WIDTH: f32 = 80.0;
const BUTTON_HEIGHT: f32 = 30.0;
const BUTTON_MARGIN: f32 = 5.0;
const TEXT_SIZE: f32 = 16.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Resource)]
pub struct SelectedMesh{
    entity: Option<Entity>,
    mesh: Option<Handle<Mesh>>,
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

pub fn setup_ui(mut commands: Commands, ) {
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
            setup_toolbar(parent);
    });

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

}
fn setup_shapes(parent: &mut ChildBuilder) {
    parent
        .spawn((create_button_bundle(), ToolbarButton))
        .with_child(create_shape_bundle("Box"));
}


// Usage in your setup function
fn setup_toolbar(parent: &mut ChildBuilder) {
    parent
        .spawn((create_button_bundle(), ToolbarButton))
        .with_child(create_text_bundle("File"));
    
    
    parent
        .spawn((create_button_bundle(), ToolbarButton))
        .with_child(create_text_bundle("Edit"));

    parent
    .spawn((create_button_bundle(), ToolbarButton))
    .with_child(create_text_bundle("View Vertices"));
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

use crate::part;

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