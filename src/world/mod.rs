use bevy::prelude::*;
use tile_index::TileIndex;

mod tile_index;
mod world_plugin;
mod world_reader;
mod world_systems;

pub use world_plugin::WorldPlugin;

#[derive(Component)]
pub struct TileOutline;

#[derive(Resource)]
pub struct GameConfiguration {
    atlas: String,
    outline: String,
    world: String,
    tile_size: u32,
    atlas_rows: u32,
    atlas_cols: u32,
}

#[derive(Component)]
pub struct WorldState {
    pub tiles: Vec<Vec<TileIndex>>,
    pub image_handle: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct Tile;
