use bevy::prelude::*;

pub const NUM_PLAYERS: usize = 2;
pub const FPS: usize = 60;
pub const MAX_PREDICTION: usize = 12;
pub const INPUT_DELAY: usize = 2;
pub const CHECK_DISTANCE: usize = 2;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {}
}
