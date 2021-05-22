use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Clone, Copy)]
pub struct MapTileType<'a> {
    pub name: &'a str,
    pub impenetrable: bool,
}

pub const DIRT_MAP_TILE: MapTileType = MapTileType {
    name: "Dirt",
    impenetrable: false,
};
pub const BUSHES_MAP_TILE: MapTileType = MapTileType {
    name: "Bushes",
    impenetrable: true,
};
pub const WALL_MAP_TILE: MapTileType = MapTileType {
    name: "Wall",
    impenetrable: true,
};
pub const LADDER_MAP_TILE: MapTileType = MapTileType {
    name: "Ladder",
    impenetrable: false,
};

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
pub struct EnemyType<'a> {
    pub name: &'a str,
    pub health: u32,
    pub speed: u32,
}

pub const CRAWLER_ENEMY_TYPE: EnemyType = EnemyType {
    name: "Crawler",
    health: 1,
    speed: 1,
};

pub const WALKER_ENEMY_TYPE: EnemyType = EnemyType {
    name: "Walker",
    health: 1,
    speed: 1,
};

pub const RUNNER_ENEMY_TYPE: EnemyType = EnemyType {
    name: "Runner",
    health: 1,
    speed: 1,
};

#[derive(Serialize, Clone)]
pub struct Enemy {
    pub id: usize,
    #[serde(rename(serialize = "typeIndex"))]
    pub type_index: usize,
    pub path: Vec<usize>,
}

#[derive(Serialize, Clone, Copy)]
pub struct PowerupType<'a> {
    pub name: &'a str,
    pub health_boost: u32,
    pub speed_boost: u32,
}

pub const SPEED_POWERUP_TYPE: PowerupType = PowerupType {
    name: "Speed",
    health_boost: 0,
    speed_boost: 1,
};

#[derive(Serialize, Clone)]
pub struct Powerup {
    #[serde(rename(serialize = "typeIndex"))]
    pub type_index: usize,
    pub index: usize,
}

#[derive(Deserialize, Clone)]
pub struct LevelRequest {
    pub level: u32,
}

#[derive(Serialize, Clone)]
pub struct LevelResponse<'a> {
    #[serde(rename(serialize = "tileTypes"))]
    pub tile_types: Vec<MapTileType<'a>>,
    pub tiles: Vec<MapTile>,
    #[serde(rename(serialize = "enemyTypes"))]
    pub enemy_types: Vec<EnemyType<'a>>,
    pub enemies: Vec<Enemy>,
    pub portals: Vec<Portal>,
    #[serde(rename(serialize = "powerupTypes"))]
    pub powerup_types: Vec<PowerupType<'a>>,
    pub powerups: Vec<Powerup>,
    #[serde(rename(serialize = "charStartIndex"))]
    pub char_start_index: usize,
}
