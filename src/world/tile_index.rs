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
    ) -> TileIndex {
        match (top_left, top_right, bottom_left, bottom_right) {
            ('G', 'G', 'G', 'W') => TileIndex::ShoreTopLeft,
            ('G', 'G', 'W', 'W') => TileIndex::ShoreTopMiddle,
            ('G', 'G', 'W', 'G') => TileIndex::ShoreTopRight,
            ('G', 'W', 'G', 'W') => TileIndex::ShoreLeft,
            ('W', 'W', 'W', 'W') => TileIndex::Water,
            ('W', 'G', 'W', 'G') => TileIndex::ShoreRight,
            ('G', 'W', 'G', 'G') => TileIndex::ShoreBottomLeft,
            ('W', 'W', 'G', 'G') => TileIndex::ShoreBottomMiddle,
            ('W', 'G', 'G', 'G') => TileIndex::ShoreBottomRight,
            ('W', 'W', 'W', 'G') => TileIndex::IslandTopLeft,
            ('W', 'W', 'G', 'W') => TileIndex::IslandTopRight,
            ('G', 'G', 'G', 'G') => TileIndex::Grass,
            ('W', 'G', 'W', 'W') => TileIndex::IslandBottomLeft,
            ('G', 'W', 'W', 'W') => TileIndex::IslandBottomRight,
            _ => unreachable!(),
        }
    }
}
