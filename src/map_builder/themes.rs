use crate::prelude::*;

pub struct DungeonTheme;

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        return Box::new(Self);
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        return match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
        };
    }
}


pub struct ForestTheme;

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        return Box::new(Self);
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        return match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
        }
    }
}