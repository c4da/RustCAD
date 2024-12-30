use bevy::prelude::*;
use crate::tools::colors::{GRAY, WHITE, RED, YELLOW, BLACK};

#[derive(Component,)]
pub struct ToolbarButton;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(( 
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
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Row,
                ..Default::default()
    }
    ,BackgroundColor(GRAY)
    ,)).with_children(|parent| {
        //FILE Button
        //This magic appends child to a node / container
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
            ToolbarButton,
        )).with_children(|parent| {
            //this appends another child to the button
            parent.spawn((
                Text::new("File"),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf").clone().into(),
                    font_size: 16.0,
                    ..Default::default()
                },
                TextColor(BLACK),
            ));
        });

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
            ToolbarButton,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Edit"),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    // font: asset_server.load("fonts/FiraSans-Bold.ttf").clone().into(),
                    font_size: 16.0,
                    ..Default::default()
                },
                TextColor(BLACK.into()),
            ));
        });

    });
    // commands.add_systems(Update, toolbar_system);
}

//this should handle button hover/click logic
pub fn toolbar_system(
    mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
    >,
    // mouse_input: Res<ButtonInput<MouseButton>>,
) {
    for (interaction, mut color) in &mut interaction_query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => 
                YELLOW.into(),
                // *color = BackgroundColor(YELLOW);
                // info!("hovered");
            
            Interaction::Pressed => RED.into(),
                // *color = BackgroundColor(RED);
                // info!("clicked");
            // }
            Interaction::None => GRAY.into(),
                // *color = BackgroundColor(GRAY);
            // }
        }
    }
}