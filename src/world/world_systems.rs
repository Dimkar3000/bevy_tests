use bevy::prelude::*;

use super::{world_reader::WorldReader, GameConfiguration, Tile, TileOutline, WorldState};

pub fn read_configuration(mut commands: Commands) {
    commands.insert_resource(GameConfiguration {
        atlas: "atlas.png".to_string(),
        outline: "outline.png".to_string(),
        world: "world.txt".to_string(),
        tile_size: 16,
        atlas_rows: 6,
        atlas_cols: 3,
    });
}

pub fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfiguration>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image_handle = asset_server.load(&game_config.atlas);
    let outline_handle = asset_server.load(&game_config.outline);
    let atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(game_config.tile_size),
        game_config.atlas_cols,
        game_config.atlas_rows,
        None,
        None,
    );
    let layout = texture_atlas_layouts.add(atlas_layout);
    commands.spawn((
        TileOutline,
        Sprite::from_image(outline_handle.clone()),
        Transform::IDENTITY,
    ));

    let reader = WorldReader::from_file(&game_config.world).unwrap_or_default();
    let world = WorldState {
        tiles: reader.into_tiles().unwrap(),
        image_handle,
        layout,
    };
    let width = game_config.tile_size as f32;
    let height = game_config.tile_size as f32;

    for (row_index, row) in world.tiles.iter().enumerate() {
        for (col_index, tile) in row.iter().enumerate() {
            commands.spawn((
                Sprite::from_atlas_image(
                    world.image_handle.clone(),
                    TextureAtlas {
                        layout: world.layout.clone(),
                        index: *tile as usize,
                    },
                ),
                Tile,
                Transform::from_xyz(col_index as f32 * height, (row_index as f32 * width), 0.),
            ));
        }
    }

    commands.spawn(world);
}

pub fn update_tile(
    windows: Query<&Window>,
    game_config: Res<GameConfiguration>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut sprites: Query<(&mut Sprite, &Transform), Without<Outline>>,
) {
    let window = windows.single();
    let (camera, position) = cameras.single();
    if let Some(world_position) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(position, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        for (mut tile, transform) in &mut sprites {
            let x = transform.translation.x + (game_config.tile_size / 2) as f32;
            let y = transform.translation.y + (game_config.tile_size / 2) as f32;
            if x >= world_position.x
                && x <= world_position.x + game_config.tile_size as f32
                && y >= world_position.y
                && y <= world_position.y + game_config.tile_size as f32
            {
                if let Some(atlas) = tile.texture_atlas.as_mut() {
                    let mut new_index = atlas.index + 1;
                    if new_index >= (game_config.atlas_rows * game_config.atlas_cols) as usize {
                        new_index = 0;
                    }
                    atlas.index = new_index;
                }
                return;
            }
        }
    }
}

pub fn move_outline(
    query: Single<&mut Transform, With<TileOutline>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    sprites: Query<&Transform, (With<Sprite>, Without<TileOutline>)>,
) {
    let mut outline = query.into_inner();
    let window = windows.single();
    let (camera, position) = cameras.single();
    if let Some(world_position) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(position, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        for tile in sprites.iter() {
            let x = tile.translation.x + 8.0;
            let y = tile.translation.y + 8.0;
            if x >= world_position.x
                && x <= world_position.x + 16.
                && y >= world_position.y
                && y <= world_position.y + 16.
            {
                outline.translation = tile.translation;
                outline.translation.z = 1.;
                return;
            }
        }
        outline.translation.z = -1.;
    }
}
