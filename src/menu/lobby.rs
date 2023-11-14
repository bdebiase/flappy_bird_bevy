use bevy::prelude::*;

use super::MenuState;

#[derive(Component)]
pub struct MenuOnlineUI;

#[derive(Component)]
pub enum MenuOnlineBtn {
    LobbyMatch,
    QuickMatch,
    Back,
}

#[derive(Component)]
pub struct ButtonEnabled(bool);

#[derive(Component)]
pub struct LobbyCodeText;

#[derive(Resource)]
pub struct LobbyID(String);

pub struct LobbyMenuPlugin;

impl Plugin for LobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LobbyID("".to_owned()))
            .add_systems(OnEnter(MenuState::Lobby), setup)
            .add_systems(OnExit(MenuState::Lobby), despawn)
            .add_systems(Update, handle_buttons.run_if(in_state(MenuState::Lobby)));
    }
}

pub fn setup(mut commands: Commands) {}

pub fn handle_buttons(
    mut commands: Commands,
    mut state: ResMut<NextState<MenuState>>,
    lobby_id: Res<LobbyID>,
    mut interaction_query: Query<
        (&Interaction, &MenuOnlineBtn, Option<&ButtonEnabled>),
        Changed<Interaction>,
    >,
) {
    for (interaction, btn, enabled) in interaction_query.iter_mut() {
        let clickable = match enabled {
            Some(e) => e.0,
            None => true,
        };

        if !clickable {
            continue;
        }

        if let Interaction::Pressed = *interaction {
            match btn {
                MenuOnlineBtn::LobbyMatch => {}
                MenuOnlineBtn::QuickMatch => {}
                MenuOnlineBtn::Back => {}
            }
        }
    }
}

pub fn despawn(query: Query<Entity, With<MenuOnlineUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
