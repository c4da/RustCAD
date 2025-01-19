pub struct AiConsolePlugin;
use crate::{part, 
            tools::{self, colors::{BORDER_COLOR, NORMAL_BUTTON}},
            ui::{self, ui_elements::{CustomTextBundle}},
        };

use bevy::{prelude::*, color::palettes::css::*, input::keyboard::KeyCode, time::Time};
use bevy_inspector_egui::egui::epaint::text::cursor;
use tools::colors;

#[derive(Component)]
struct CursorBlink {
    visible: bool,
    timer: Timer,
}

#[derive(Component, Resource, Default)]
struct ConsoleInput(String);

impl Plugin for AiConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleInput(String::new()))
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
    keys: Res<ButtonInput<KeyCode>>,
    mut text_query: Query<&mut Text, With<ConsoleInputField>>,
    mut console_input: ResMut<ConsoleInput>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            console_input.0 = text.0.clone();
            text.0.clear();
            process_console_command(&console_input.0, &mut commands, &mut meshes, &mut materials);
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
        // let number_keys = [
        //     (KeyCode::Numpad1, '1'), (KeyCode::Key2, '2'), (KeyCode::Key3, '3'),
        //     (KeyCode::Numpad4, '4'), (KeyCode::Key5, '5'), (KeyCode::Key6, '6'),
        //     (KeyCode::Numpad7, '7'), (KeyCode::Key8, '8'), (KeyCode::Key9, '9'),
        //     (KeyCode::Key0, '0'),
        // ];

        // for (key, c) in number_keys.iter() {
        //     if keys.just_pressed(*key) {
        //         text.0.push(*c);
        //     }
        // }

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
                *color = BackgroundColor::from(Color::rgb(0.35, 0.35, 0.35));
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(Color::rgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor::from(NORMAL_BUTTON);
            }
        }
    }
}

fn setup_console_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
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
        BackgroundColor::from(Color::rgb(0.1, 0.1, 0.1)),
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
            BackgroundColor::from(Color::rgb(0.15, 0.15, 0.15)),
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
