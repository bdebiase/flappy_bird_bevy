mod anchor;
mod animation;
mod background;
mod game;
mod ground;
mod menu;
mod networking;
mod pipes;
mod player;
mod tiling;
mod util;

use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resolution: (480.0, 720.0).into(),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        .run();
}
