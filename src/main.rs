mod anchor;
mod animation;
mod level;
mod game;
mod menu;
mod physics;
mod pipes;
mod player;
mod tiling;
mod camera;

use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use game::GamePlugins;

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
            FramepacePlugin,
            GamePlugins,
        ))
        // .insert_resource(FramepaceSettings::default().with_limiter(Limiter::from_framerate(60.0)))
        .run();
}
