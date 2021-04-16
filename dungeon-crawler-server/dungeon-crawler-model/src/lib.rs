use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Clone, Copy)]
pub enum MapTileTypeName {
    Dirt,
    Bushes,
}

#[derive(Serialize, Clone, Copy)]
pub struct MapTileType {
    pub name: MapTileTypeName,
    pub impenetrable: bool,
}

#[derive(Serialize, Clone, Copy)]
pub struct MapTile {
    #[serde(rename(serialize = "typeIndex"))]
    pub type_index: usize,
    pub index: usize,
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

#[derive(Serialize, Clone, Copy)]
pub enum EnemyTypeName {
    Crawler,
    Walker,
}

#[derive(Serialize, Clone, Copy)]
pub struct EnemyType {
    pub name: EnemyTypeName,
    pub health: u32,
    pub speed: u32,
}

#[derive(Serialize, Clone)]
pub struct Enemy {
    pub id: usize,
    #[serde(rename(serialize = "typeIndex"))]
    pub type_index: usize,
    pub path: Vec<usize>,
}

#[derive(Deserialize, Clone)]
pub struct LevelRequest {
    pub level: u32,
}

#[derive(Serialize, Clone)]
pub struct LevelResponse {
    #[serde(rename(serialize = "charStartIndex"))]
    pub char_start_index: usize,
    #[serde(rename(serialize = "tileTypes"))]
    pub tile_types: Vec<MapTileType>,
    pub tiles: Vec<MapTile>,
    #[serde(rename(serialize = "enemyTypes"))]
    pub enemy_types: Vec<EnemyType>,
    pub enemies: Vec<Enemy>,
    pub portals: Vec<Portal>,
}
