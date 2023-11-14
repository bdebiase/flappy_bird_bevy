use bevy::{app::AppExit, prelude::*};

use crate::game::{GameAssets, GameState};

use super::MenuState;

#[derive(Component)]
pub struct MenuMainUI;

#[derive(Component)]
pub enum MenuMainBtn {}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Main), setup)
            .add_systems(OnExit(MenuState::Main), despawn)
            .add_systems(Update, handle_buttons);
    }
}

pub fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {}

pub fn handle_buttons(
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: Query<(&Interaction, &MenuMainBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {}
    }
}

pub fn despawn(query: Query<Entity, With<MenuMainUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
