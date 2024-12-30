use bevy::prelude::*;
use crate::tools::colors::{GRAY, PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, RED};

#[derive(Component,)]
pub struct ToolbarButton;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        //FILE Button
        //This magic appends child to a node / container
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(30.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(GRAY),
                BorderColor(Color::BLACK),
                ToolbarButton,
            )).with_child((
                Text::new("File"),
                TextFont {
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent.spawn((
                Button,
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(30.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(GRAY),
                BorderColor(Color::BLACK),
                ToolbarButton,
            )).with_child((
                Text::new("Edit"),
                TextFont {
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

    });

}

pub fn button_system(
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