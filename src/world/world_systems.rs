use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{error::GameError, prelude::Result};

use super::{tile_index::TileIndex, Tile, TileOutline, WorldState};

pub fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image_handle = asset_server.load("atlas.png");
    let outline_handle = asset_server.load("outline.png");
    let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 6, None, None);
    let layout = texture_atlas_layouts.add(atlas_layout);
    commands.spawn((
        TileOutline,
        Sprite::from_image(outline_handle.clone()),
        Transform::IDENTITY,
    ));
    let world = WorldState {
        tiles: from_file("world.txt").unwrap(),
        image_handle,
        layout,
    };
    let width = 16.;
    let height = 16.;

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
                Transform::from_xyz(col_index as f32 * height, -(row_index as f32 * width), 0.),
            ));
        }
    }

    commands.spawn(world);
}

pub fn update_tile(
    windows: Query<&Window>,
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
            let x = transform.translation.x + 8.0;
            let y = transform.translation.y + 8.0;
            if x >= world_position.x
                && x <= world_position.x + 16.
                && y >= world_position.y
                && y <= world_position.y + 16.
            {
                tile.texture_atlas.as_mut().unwrap().index += 1;
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

fn from_file(filename: &str) -> Result<Vec<Vec<TileIndex>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|x| x.ok().map(|x| x.chars().collect()))
        .collect();

    let height = lines.len();
    if height == 0 {
        return Err(GameError("Map was empty".to_string()));
    }

    let width = lines[0].len();
    if width == 0 {
        return Err(GameError("Width of the map is 0".to_string()));
    }
    let mut result = Vec::with_capacity(height / 2);
    for row in (0..height).step_by(2) {
        let mut row_vec = Vec::with_capacity(width / 2);
        for col in (0..width).step_by(2) {
            let top_left = lines[row][col];
            let top_right = lines[row][col + 1];
            let bottom_left = lines[row + 1][col];
            let bottom_right = lines[row + 1][col + 1];
            row_vec.push(TileIndex::new(
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            )?);
        }
        result.push(row_vec);
    }

    Ok(result)
}
