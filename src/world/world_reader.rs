use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{error::GameError, prelude::*};

use super::tile_index::TileIndex;

#[derive(Debug)]
pub struct WorldReader {
    pub width: usize,
    pub height: usize,
    pub base_row: usize,
    pub base_col: usize,
    pub save_path: Option<String>,
}

impl Default for WorldReader {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            base_row: Default::default(),
            base_col: Default::default(),
            save_path: Default::default(),
        }
    }
}

impl WorldReader {
    pub fn into_tiles(self) -> Result<Vec<Vec<TileIndex>>> {
        let mut result = (0..self.height)
            .map(|_row| (0..self.width).map(|_width| TileIndex::Grass).collect())
            .collect();

        if let Some(path) = self.save_path {
            result = from_file(&path, self.base_row, self.base_col, result)?;
        }

        Ok(result)
    }

    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let file = File::open(path.into())?;
        let reader = BufReader::new(file);

        let mut world = WorldReader::default();

        let mut lines = reader.lines();

        // read size
        if let Some(line) = lines.next() {
            let data = line?;
            if let Some((height, width)) = data.split_once(',') {
                world.height = height.parse()?;
                world.width = width.parse()?;
            } else {
                return Err(GameError::new("failed to parse world width and size"));
            }
        } else {
            return Err(GameError::new("failed to read World file"));
        }

        // read base
        if let Some(line) = lines.next() {
            let data = line?;
            if let Some((row, col)) = data.split_once(',') {
                world.base_row = row.parse()?;
                world.base_col = col.parse()?;
            } else {
                return Err(GameError::new("failed to parse world width and size"));
            }
        } else {
            return Err(GameError::new("failed to read World file"));
        }

        world.save_path = lines.next().map(|x| x.unwrap_or_default());
        dbg!(&world);
        Ok(world)
    }
}

fn from_file(
    filename: &str,
    base_row: usize,
    base_col: usize,
    mut base_map: Vec<Vec<TileIndex>>,
) -> Result<Vec<Vec<TileIndex>>> {
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

    for row in (0..height).step_by(2) {
        for col in (0..width).step_by(2) {
            let top_left = lines[row][col];
            let top_right = lines[row][col + 1];
            let bottom_left = lines[row + 1][col];
            let bottom_right = lines[row + 1][col + 1];
            base_map[base_row + row / 2][base_col + col / 2] =
                TileIndex::new(top_left, top_right, bottom_left, bottom_right)?
        }
    }

    Ok(base_map)
}
