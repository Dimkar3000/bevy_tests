use bevy::prelude::*;
use camera::GameCameraPlugin;
use world::WorldPlugin;

mod camera;
mod error;
mod prelude;
mod world;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::Mailbox,
                        name: Some("Fishing is Boring".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(WorldPlugin)
        .add_plugins(GameCameraPlugin)
        .run();
}
