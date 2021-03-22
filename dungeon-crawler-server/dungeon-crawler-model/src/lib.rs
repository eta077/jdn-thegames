use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Clone, Copy)]
pub enum MapTileType {
    Dirt,
    Bushes,
}

#[derive(Serialize, Clone, Copy)]
pub struct MapTile {
    pub index: usize,
    pub r#type: MapTileType,
}

#[derive(Serialize, Clone, Copy)]
pub enum PortalOrientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Clone, Copy)]
pub struct Portal {
    pub index: usize,
    pub target: Option<u32>,
    pub orientation: PortalOrientation,
}

#[derive(Deserialize, Clone)]
pub struct LevelRequest {
    pub level: u32,
}

#[derive(Serialize, Clone)]
pub struct LevelResponse {
    pub char_start_index: usize,
    pub tiles: Vec<MapTile>,
    pub portals: Vec<Portal>,
}
