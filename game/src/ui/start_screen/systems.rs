// bevy
use bevy::prelude::*;

// resources
use crate::resources::fonts::GameFonts;

pub fn setup_start_screen(mut commands: Commands, game_fonts: Res<GameFonts>) {
    // Spawn a 2D camera for UI rendering
    commands.spawn(Camera2d);

    // Root UI node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|parent| {
            // Title text
            parent.spawn((
                Text::new("Soul Eaters"),
                TextFont {
                    font: game_fonts.josefin_sans_bold.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Start button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::top(Val::Px(20.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start Game"),
                        TextFont {
                            font: game_fonts.josefin_sans_regular.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}