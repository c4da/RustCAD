pub struct AiConsolePlugin;
use tokio::runtime::Runtime;
use serde_json::Value;

use crate::{ai::{
                ai_client::AiClient,
                json_parser::{self, LlmCubeCommand},
                secretive_secret::API_KEY,
            },
            part::{self, primitives},
            tools::colors::{HOVERED_BUTTON_COLOR,
                            NEAR_BLACK,
                            NORMAL_BUTTON_COLOR,
                            PRESSED_BUTTON_COLOR},
            ui::ui_elements::CustomTextBundle
        };

use bevy::{input::keyboard::KeyCode, prelude::*, time::Time};
use bevy::{prelude::*, time::Time};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};


#[derive(Component)]
struct CursorBlink {
    visible: bool,
    timer: Timer,
}

#[derive(Component, Resource, Default, Clone)]
struct ConsoleInput(String, bool);

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

        app.insert_resource(ConsoleInput(String::new(), false))
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
                process_console_ai_command(&response, &mut commands, &mut meshes, &mut materials);
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
    mut evr_kbd: EventReader<KeyboardInput>,
    mut text_query: Query<&mut Text, With<ConsoleInputField>>,
    mut console_input: ResMut<ConsoleInput>,
    ai_client: Res<AiClient>,
    tasks: Query<Entity, With<AsyncApiTask>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        for ev in evr_kbd.read() {
            if ev.state == ButtonState::Released {
                if ev.logical_key == Key::Control {
                    console_input.1 = false;
                }
                continue;
            }
            match &ev.logical_key {
                Key::Enter => {
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
                Key::Backspace => {
                    text.0.pop();
                    if console_input.1 {
                        text.0.clear();
                    }
                }
                Key::Space => {
                    text.0.push(' ');
                }
                // Handle key presses that produce text characters
                Key::Character(input) => {
                    // Ignore any input that contains control (special) characters
                    if console_input.1 {
                        println!("Skipping due to Ctrl {:?}", input);
                        continue;
                    }
                    text.0.push_str(&input);
                }
                Key::Control => {
                    console_input.1 = true;
                }
                _ => {}
            }
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
                *color = BackgroundColor(PRESSED_BUTTON_COLOR);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *color = BackgroundColor(NORMAL_BUTTON_COLOR);
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
        BackgroundColor(NEAR_BLACK),
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
            BackgroundColor(NORMAL_BUTTON_COLOR),
            BorderColor::from(NEAR_BLACK),
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
            BackgroundColor(NEAR_BLACK),
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
    let mut points = primitives::CubePoints::get_points();

    for point in &mut points {
        *point += command.get_vector_from_origin();
    }

    part::create_3d_object_system(commands, meshes, materials, points);
}

fn create_cubes_from_command(command: &Vec<LlmCubeCommand>, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
    println!("Creating cubes");
    let points = primitives::CubePoints::get_points();

    command.iter().for_each(|cube_command| {
        let mut cube_pos = points.clone();
        for point in &mut cube_pos {
            *point += cube_command.get_vector_from_origin();
        }
        part::create_3d_object_system(commands, meshes, materials, cube_pos);
        print!("Created cube at: {:?}", cube_command.get_vector_from_origin());
    });
}

fn process_console_ai_command(
    llm_response: &String,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    let full_command: Value = serde_json::from_str(llm_response).unwrap();
    let command: &str = Box::leak(full_command["command"].as_str().unwrap().to_string().into_boxed_str());
    match command {
        "create cube" => {
            let command = json_parser::parse_cube_command(&llm_response);
            create_cube_from_command(&command, commands, meshes, materials);
            println!("Created cube");
        }
        "create cubes" => {
            let command = json_parser::parse_cubes_command(&llm_response);
            create_cubes_from_command(&command, commands, meshes, materials);
            println!("Cubes created");
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
