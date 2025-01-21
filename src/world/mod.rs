use bevy::prelude::*;
use tile_index::TileIndex;

mod tile_index;
mod world_systems;

mod world_plugin;
pub use world_plugin::WorldPlugin;

#[derive(Component)]
pub struct TileOutline;

#[derive(Component)]
pub struct WorldState {
    pub tiles: Vec<Vec<TileIndex>>,
    pub image_handle: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct Tile;
