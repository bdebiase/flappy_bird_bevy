use bevy::prelude::*;

use crate::util;

use super::MenuState;

#[derive(Component)]
pub struct MenuConnectUI;

#[derive(Component)]
pub enum MenuConnectBtn {}

pub struct ConnectMenuPlugin;

impl Plugin for ConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Connect), setup)
            .add_systems(OnExit(MenuState::Connect), util::despawn::<MenuConnectUI>)
            .add_systems(Update, handle_buttons);
    }
}

pub fn setup(mut commands: Commands) {}

pub fn handle_buttons(
    mut state: ResMut<NextState<MenuState>>,
    mut interaction_query: Query<(&Interaction, &MenuConnectBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {}
    }
}
