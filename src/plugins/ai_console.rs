pub struct AiConsolePlugin;
use crate::{part, 
            tools::{colors::{BORDER_COLOR, NORMAL_BUTTON}},
            ui::{ui_elements::{CustomTextBundle}},
        };

use bevy::{prelude::*, time::Time};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};


#[derive(Component)]
struct CursorBlink {
    visible: bool,
    timer: Timer,
}

#[derive(Component, Resource, Default)]
struct ConsoleInput(String, bool);

impl Plugin for AiConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleInput(String::new(), false))
            .add_systems(Startup, setup_console_ui)
            .add_systems(Update, (
                handle_console_input,
                button_interaction_system,
                keyboard_input_system,
                cursor_blink_system,
            ));
    }
}

fn keyboard_input_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut text_query: Query<&mut Text, With<ConsoleInputField>>,
    mut console_input: ResMut<ConsoleInput>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                    console_input.0 = text.0.clone();
                    text.0.clear();
                    process_console_command(&console_input.0, &mut commands, &mut meshes, &mut materials);
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
                *color = BackgroundColor::from(Color::srgb(0.35, 0.35, 0.35));
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor::from(NORMAL_BUTTON);
            }
        }
    }
}

fn setup_console_ui(mut commands: Commands) {
    // commands.spawn(UiCameraBundle::default());

    // Console container
    let mut node = Node::default();
    node.position_type = PositionType::Absolute;
    node.bottom = Val::Px(20.0);
    node.left = Val::Percent(30.0); // Center by starting at 30%
    node.width = Val::Percent(40.0);
    node.height = Val::Px(40.0);
    node.flex_direction = FlexDirection::Column;

    commands.spawn((
        node,
        BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
        PickingBehavior::IGNORE,
    ))
    .with_children(|parent| {
        // Text input field container
        let mut input_node = Node::default();
        input_node.width = Val::Percent(40.0);
        input_node.height = Val::Px(40.0);
        input_node.margin = UiRect::all(Val::Px(5.0));
        input_node.padding = UiRect::horizontal(Val::Px(8.0));
        input_node.justify_content = JustifyContent::FlexStart;
        input_node.align_items = AlignItems::Center;
        input_node.border = UiRect::all(Val::Px(1.0));
        input_node.align_self = AlignSelf::Center;

        parent.spawn((
            input_node,
            BackgroundColor::from(Color::srgb(0.15, 0.15, 0.15)),
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
        button_node.width = Val::Px(100.0);
        button_node.height = Val::Px(30.0);
        button_node.margin = UiRect::all(Val::Px(5.0));
        button_node.padding = UiRect::horizontal(Val::Px(8.0));
        button_node.justify_content = JustifyContent::FlexStart;
        button_node.align_items = AlignItems::Center;

        parent.spawn((
            Button,
            button_node,
            BackgroundColor::from(NORMAL_BUTTON),
            BorderColor::from(BORDER_COLOR),
            ConsoleSubmitButton,
        ))
        .with_children(|parent| {
            parent.spawn(CustomTextBundle::new("Submit", 20.0));
        });
    });
}

#[derive(Component)]
struct ConsoleInputField;

#[derive(Component)]
struct ConsoleSubmitButton;

fn handle_console_input(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &ConsoleSubmitButton), Changed<Interaction>>,
    mut input_query: Query<&mut Text, With<ConsoleInputField>>,
    mut console_input: ResMut<ConsoleInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (interaction, _) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut text) = input_query.get_single_mut() {
                // Get the current text value and store it
                console_input.0 = text.0.clone();
                
                // Clear the input field
                text.0.clear();
                
                // Process the command
                process_console_command(&console_input.0, &mut commands, &mut meshes, &mut materials);
        }
    }
    }
}

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

fn process_console_command(
    command: &str, 
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    let command = command.trim();
    
    match command {
        "create cube" => {
            let points = vec![
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(2.0, 1.0, 1.0),
                Vec3::new(2.0, 2.0, 1.0),
                Vec3::new(1.0, 2.0, 1.0),
                Vec3::new(1.0, 1.0, 2.0),
                Vec3::new(2.0, 1.0, 2.0),
                Vec3::new(2.0, 2.0, 2.0),
                Vec3::new(1.0, 2.0, 2.0),
            ];

            part::create_3d_object_system(commands, meshes, materials, points);
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
