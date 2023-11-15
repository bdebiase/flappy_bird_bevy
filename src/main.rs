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

use anchor::AnchorPlugin;
use animation::AnimationPlugin;
use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy_camera_shake::CameraShakePlugin;
use bevy_xpbd_2d::plugins::{PhysicsPlugins, PhysicsDebugPlugin};
use game::GamePlugin;
use ground::GroundPlugin;
use menu::MenuPlugin;
use pipes::PipesPlugin;
use player::PlayerPlugin;
use tiling::TilingPlugin;

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

            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            CameraShakePlugin,
            AnimationPlugin,
            AnchorPlugin,
            TilingPlugin,
            MenuPlugin,
            BackgroundPlugin,
            GroundPlugin,
            PlayerPlugin,
            PipesPlugin,
        ))
        .run();
}
