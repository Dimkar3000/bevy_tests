use crate::{error::GameError, prelude::Result};

#[derive(Debug, Clone, Copy)]
pub enum TileIndex {
    ShoreTopLeft = 0,
    ShoreTopMiddle = 1,
    ShoreTopRight = 2,
    ShoreLeft = 3,
    Water = 4,
    ShoreRight = 5,
    ShoreBottomLeft = 6,
    ShoreBottomMiddle = 7,
    ShoreBottomRight = 8,
    IslandTopLeft = 9,
    IslandTopRight = 10,
    Grass = 11,
    IslandBottomLeft = 12,
    IslandBottomRight = 13,
}

impl TileIndex {
    pub fn new(
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Result<TileIndex> {
        match (top_left, top_right, bottom_left, bottom_right) {
            ('G', 'G', 'G', 'W') => Ok(TileIndex::ShoreTopLeft),
            ('G', 'G', 'W', 'W') => Ok(TileIndex::ShoreTopMiddle),
            ('G', 'G', 'W', 'G') => Ok(TileIndex::ShoreTopRight),
            ('G', 'W', 'G', 'W') => Ok(TileIndex::ShoreLeft),
            ('W', 'W', 'W', 'W') => Ok(TileIndex::Water),
            ('W', 'G', 'W', 'G') => Ok(TileIndex::ShoreRight),
            ('G', 'W', 'G', 'G') => Ok(TileIndex::ShoreBottomLeft),
            ('W', 'W', 'G', 'G') => Ok(TileIndex::ShoreBottomMiddle),
            ('W', 'G', 'G', 'G') => Ok(TileIndex::ShoreBottomRight),
            ('W', 'W', 'W', 'G') => Ok(TileIndex::IslandTopLeft),
            ('W', 'W', 'G', 'W') => Ok(TileIndex::IslandTopRight),
            ('G', 'G', 'G', 'G') => Ok(TileIndex::Grass),
            ('W', 'G', 'W', 'W') => Ok(TileIndex::IslandBottomLeft),
            ('G', 'W', 'W', 'W') => Ok(TileIndex::IslandBottomRight),
            _ => Err(GameError::new("Tile index without a correct format")),
        }
    }
}
