// bevy
use bevy::prelude::*;

// resources
use crate::resources::fonts::GameFonts;

// states
use crate::states::GameState;

// components
use super::components::StartScreenUI;

pub fn setup_start_screen(mut commands: Commands, game_fonts: Res<GameFonts>) {
    // Spawn a 2D camera for UI rendering
    commands.spawn((Camera2d, StartScreenUI));

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
            StartScreenUI,
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

pub fn handle_start_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.1, 0.1, 0.6));
                next_state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.9));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.8));
            }
        }
    }
}

pub fn cleanup_start_screen(mut commands: Commands, query: Query<Entity, With<StartScreenUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}