use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::world_systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (read_configuration, create_world).chain())
            .add_systems(FixedUpdate, move_outline)
            .add_systems(
                Update,
                (update_tile).run_if(input_just_pressed(MouseButton::Left)),
            );
    }
}
