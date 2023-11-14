mod connect;
mod lobby;
mod main;

use bevy::prelude::*;

use crate::game::{GameScore, GameState};

use self::{connect::ConnectMenuPlugin, lobby::LobbyMenuPlugin, main::MainMenuPlugin};

const DISABLED_BUTTON: Color = Color::rgb(0.8, 0.5, 0.5);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub struct ScoreText;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum MenuState {
    #[default]
    None,
    Main,
    Connect,
    Lobby,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_plugins(MainMenuPlugin)
            .add_plugins(ConnectMenuPlugin)
            .add_plugins(LobbyMenuPlugin)
            .add_systems(OnExit(GameState::Loading), setup)
            .add_systems(
                Update,
                (
                    button_visuals,
                    update_score.run_if(resource_changed::<GameScore>()),
                ),
            );
    }
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<MenuState>>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font_size: 64.0,
                    ..default()
                },
            ),
            style: Style {
                justify_self: JustifySelf::Center,
                top: Val::Percent(10.0),
                ..default()
            },
            ..default()
        },
        ScoreText,
    ));

    next_state.set(MenuState::Main);
}

fn update_score(mut query: Query<&mut Text, With<ScoreText>>, game_score: Res<GameScore>) {
    query.for_each_mut(|mut text| {
        text.sections.get_mut(0).unwrap().value = game_score.to_string();
    });
}

pub fn button_visuals(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
