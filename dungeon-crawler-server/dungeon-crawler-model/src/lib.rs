use std::fmt;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Clone, Copy)]
pub enum MapTileType {
    Dirt,
    Bushes,
}

impl fmt::Display for MapTileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match self {
            MapTileType::Dirt => "dirt",
            MapTileType::Bushes => "bushes",
        };
        write!(f, "{}", r)
    }
}

#[derive(Serialize, Clone, Copy)]
pub struct MapTile {
    pub index: usize,
    pub r#type: MapTileType,
}

#[derive(Deserialize, Clone)]
pub struct LevelRequest {
    pub level: u32,
}

#[derive(Serialize, Clone)]
pub struct LevelResponse {
    pub char_start_index: usize,
    pub tiles: Vec<MapTile>,
}
