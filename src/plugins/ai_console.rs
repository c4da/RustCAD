pub struct AiConsolePlugin;
use futures::channel::mpsc;
use futures::SinkExt;
use tokio::runtime::Runtime;
use std::future::Future;

use crate::{ai::{
                ai_client::AiClient,
                json_parser::{self, LlmCubeCommand},
                secretive_secret::API_KEY,
            },
            part, 
            tools::{colors::{BORDER_COLOR, NORMAL_BUTTON}},
            ui::{ui_elements::CustomTextBundle}
        };

use bevy::{input::keyboard::KeyCode, prelude::*, time::Time};

#[derive(Component)]
struct CursorBlink {
    visible: bool,
    timer: Timer,
}

#[derive(Component, Resource, Default, Clone)]
struct ConsoleInput(String);

#[derive(Resource)]
struct AsyncRuntime(Runtime);

// Component to store the async task
#[derive(Component)]
struct AsyncApiTask {
    input: String,
    client: AiClient,
}

impl Plugin for AiConsolePlugin {
    fn build(&self, app: &mut App) {
        // Create a single runtime for async operations
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        
        app.insert_resource(ConsoleInput(String::new()))
            .insert_resource(AiClient::new(API_KEY.to_string()))
            .insert_resource(AsyncRuntime(runtime))
            .add_systems(Startup, setup_console_ui)
            .add_systems(Update, (
                button_interaction_system,
                keyboard_input_system,
                cursor_blink_system,
                handle_api_response,
            ));
    }
}

fn handle_api_response(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tasks: Query<(Entity, &AsyncApiTask)>,
    runtime: Res<AsyncRuntime>,
) {
    for (entity, task) in tasks.iter() {
        let result = runtime.0.block_on(async {
            task.client.call_llm_api(&task.input).await
        });

        match result {
            Ok(response) => {
                let command = json_parser::parse_llm_output(&response);
                process_console_ai_command(&command, &mut commands, &mut meshes, &mut materials);
            }
            Err(e) => {
                println!("Error calling LLM API: {:?}", e);
            }
        }

        commands.entity(entity).despawn();
    }
}

fn keyboard_input_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut text_query: Query<&mut Text, With<ConsoleInputField>>,
    mut console_input: ResMut<ConsoleInput>,
    ai_client: Res<AiClient>,
    tasks: Query<Entity, With<AsyncApiTask>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        // Handle backspace
        if keys.just_pressed(KeyCode::Backspace) {
            if !text.0.is_empty() {
                text.0.pop();
            }
            return;
        }

        // Handle Enter key
        if keys.just_pressed(KeyCode::Enter) {
            // Only process if there's no pending task
            if tasks.iter().next().is_none() {
                println!("keyboard_input_system -> Submitting console input: {}", text.0);
                console_input.0 = text.0.clone();
                text.0.clear();

                let task = AsyncApiTask {
                    input: console_input.0.clone(),
                    client: ai_client.clone(),
                };

                // Spawn an entity to track the task
                commands.spawn(task);
            }
            return;
        }

        // Handle text input
        let shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

        // Handle letters
        let letter_keys = [
            (KeyCode::KeyA, 'a'), (KeyCode::KeyB, 'b'), (KeyCode::KeyC, 'c'),
            (KeyCode::KeyD, 'd'), (KeyCode::KeyE, 'e'), (KeyCode::KeyF, 'f'),
            (KeyCode::KeyG, 'g'), (KeyCode::KeyH, 'h'), (KeyCode::KeyI, 'i'),
            (KeyCode::KeyJ, 'j'), (KeyCode::KeyK, 'k'), (KeyCode::KeyL, 'l'),
            (KeyCode::KeyM, 'm'), (KeyCode::KeyN, 'n'), (KeyCode::KeyO, 'o'),
            (KeyCode::KeyP, 'p'), (KeyCode::KeyQ, 'q'), (KeyCode::KeyR, 'r'),
            (KeyCode::KeyS, 's'), (KeyCode::KeyT, 't'), (KeyCode::KeyU, 'u'),
            (KeyCode::KeyV, 'v'), (KeyCode::KeyW, 'w'), (KeyCode::KeyX, 'x'),
            (KeyCode::KeyY, 'y'), (KeyCode::KeyZ, 'z'),
        ];

        for (key, c) in letter_keys.iter() {
            if keys.just_pressed(*key) {
                if shift_pressed {
                    text.0.push(c.to_ascii_uppercase());
                } else {
                    text.0.push(*c);
                }
            }
        }

        // Handle numbers
        let number_keys = [
            (KeyCode::Numpad1, '1'), (KeyCode::Numpad2, '2'), (KeyCode::Numpad3, '3'),
            (KeyCode::Numpad4, '4'), (KeyCode::Numpad5, '5'), (KeyCode::Numpad6, '6'),
            (KeyCode::Numpad7, '7'), (KeyCode::Numpad8, '8'), (KeyCode::Numpad9, '9'),
            (KeyCode::Numpad0, '0'),
        ];

        for (key, c) in number_keys.iter() {
            if keys.just_pressed(*key) {
                text.0.push(*c);
            }
        }

        // Handle special characters
        if keys.just_pressed(KeyCode::Space) {
            text.0.push(' ');
        }
    }
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(NORMAL_BUTTON);
            }
        }
    }
}

fn setup_console_ui(mut commands: Commands) {
    let mut node = Node::default();
    node.position_type = PositionType::Absolute;
    node.bottom = Val::Px(20.0);
    node.left = Val::Percent(33.0);
    node.width = Val::Percent(33.0);
    node.height = Val::Px(45.0);
    node.flex_direction = FlexDirection::Row;

    commands.spawn((
        node,
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        PickingBehavior::IGNORE,
    ))
    .with_children(|parent| {
        // Text input field container
        let mut input_node = Node::default();
        input_node.width = Val::Percent(80.0);
        input_node.height = Val::Px(40.0);
        input_node.margin = UiRect::all(Val::Px(5.0));
        input_node.padding = UiRect::horizontal(Val::Px(8.0));
        input_node.justify_content = JustifyContent::FlexStart;
        input_node.align_items = AlignItems::Center;
        input_node.border = UiRect::all(Val::Px(1.0));
        input_node.align_self = AlignSelf::Center;

        parent.spawn((
            input_node,
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor::from(BORDER_COLOR),
        ))
        .with_children(|parent| {
            // Text input field
            parent.spawn((
                CustomTextBundle::new("", 20.0),
                ConsoleInputField,
            ));
            
            // Blinking cursor
            parent.spawn((
                CustomTextBundle::new("|", 20.0),
                CursorBlink {
                    visible: true,
                    timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                },
            ));
        });

        // Submit button
        let mut button_node = Node::default();
        button_node.width = Val::Px(80.0);
        button_node.height = Val::Px(30.0);
        button_node.margin = UiRect::all(Val::Px(5.0));
        button_node.padding = UiRect::horizontal(Val::Px(8.0));
        button_node.justify_content = JustifyContent::FlexStart;
        button_node.align_items = AlignItems::Center;

        parent.spawn((
            Button,
            button_node,
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ConsoleSubmitButton,
        ))
        .with_children(|parent| {
            parent.spawn(CustomTextBundle::new("Submit", 10.0));
        });
    });
}

#[derive(Component)]
struct ConsoleInputField;

#[derive(Component)]
struct ConsoleSubmitButton;

fn cursor_blink_system(
    time: Res<Time>,
    mut query: Query<(&mut CursorBlink, &mut Visibility)>,
) {
    for (mut blink, mut visibility) in query.iter_mut() {
        blink.timer.tick(time.delta());
        
        if blink.timer.just_finished() {
            blink.visible = !blink.visible;
            *visibility = if blink.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn create_cube_from_command(command: &LlmCubeCommand, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
    let mut points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
    ];

    for point in &mut points {
        *point += command.get_vector_from_origin();
    }

    part::create_3d_object_system(commands, meshes, materials, points);
}

fn process_console_ai_command(
    command: &LlmCubeCommand,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    match command.get_command() {
        "create cube" => {
            create_cube_from_command(command, commands, meshes, materials);
            println!("Created cube");
        }
        "help" => {
            println!("Available commands:");
            println!("  create cube - Creates a cube");
            println!("  help        - Shows this help message");
        }
        "" => {
            println!("Type 'help' for available commands");
        }
        _ => {
            println!("Unknown command: '{}'. Type 'help' for available commands.", command);
        }
    }
}
