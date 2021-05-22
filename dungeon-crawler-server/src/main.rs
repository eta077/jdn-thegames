use std::error::Error;

use lambda_runtime::error::HandlerError;
use lambda_runtime::lambda;
use lambda_runtime::Context;

use log::error;

use simple_logger::SimpleLogger;

use dungeon_crawler_model::*;

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    lambda!(level_handler);

    Ok(())
}

fn level_handler<'a>(e: LevelRequest, context: Context) -> Result<LevelResponse<'a>, HandlerError> {
    let level = e.level;
    match level {
        1 => Ok(LevelResponse {
            tile_types: vec![BUSHES_MAP_TILE, DIRT_MAP_TILE],
            tiles: vec![
                MapTile {
                    type_index: 0,
                    index: 0,
                },
                MapTile {
                    type_index: 0,
                    index: 1,
                },
                MapTile {
                    type_index: 0,
                    index: 2,
                },
                MapTile {
                    type_index: 0,
                    index: 3,
                },
                MapTile {
                    type_index: 0,
                    index: 4,
                },
                MapTile {
                    type_index: 1,
                    index: 5,
                },
                MapTile {
                    type_index: 1,
                    index: 6,
                },
                MapTile {
                    type_index: 1,
                    index: 7,
                },
                MapTile {
                    type_index: 0,
                    index: 8,
                },
                MapTile {
                    type_index: 0,
                    index: 9,
                },
                MapTile {
                    type_index: 0,
                    index: 10,
                },
                MapTile {
                    type_index: 0,
                    index: 11,
                },
                MapTile {
                    type_index: 1,
                    index: 12,
                },
                MapTile {
                    type_index: 0,
                    index: 13,
                },
                MapTile {
                    type_index: 0,
                    index: 14,
                },
                MapTile {
                    type_index: 0,
                    index: 15,
                },
                MapTile {
                    type_index: 0,
                    index: 16,
                },
                MapTile {
                    type_index: 1,
                    index: 17,
                },
                MapTile {
                    type_index: 1,
                    index: 18,
                },
                MapTile {
                    type_index: 1,
                    index: 19,
                },
                MapTile {
                    type_index: 0,
                    index: 20,
                },
                MapTile {
                    type_index: 0,
                    index: 21,
                },
                MapTile {
                    type_index: 0,
                    index: 22,
                },
                MapTile {
                    type_index: 0,
                    index: 23,
                },
                MapTile {
                    type_index: 0,
                    index: 24,
                },
            ],
            enemy_types: vec![WALKER_ENEMY_TYPE],
            enemies: vec![Enemy {
                id: 0,
                type_index: 0,
                path: vec![19, 18, 17, 12, 7],
            }],
            portals: vec![
                Portal {
                    index: 5,
                    target: None,
                    orientation: PortalOrientation::Right,
                },
                Portal {
                    index: 19,
                    target: Some(2),
                    orientation: PortalOrientation::Left,
                },
            ],
            powerup_types: vec![],
            powerups: vec![],
            char_start_index: 5,
        }),
        2 => Ok(LevelResponse {
            tile_types: vec![BUSHES_MAP_TILE, DIRT_MAP_TILE],
            tiles: vec![
                MapTile {
                    type_index: 0,
                    index: 0,
                },
                MapTile {
                    type_index: 0,
                    index: 1,
                },
                MapTile {
                    type_index: 0,
                    index: 2,
                },
                MapTile {
                    type_index: 0,
                    index: 3,
                },
                MapTile {
                    type_index: 0,
                    index: 4,
                },
                MapTile {
                    type_index: 0,
                    index: 5,
                },
                MapTile {
                    type_index: 0,
                    index: 6,
                },
                MapTile {
                    type_index: 0,
                    index: 7,
                },
                MapTile {
                    type_index: 0,
                    index: 8,
                },
                MapTile {
                    type_index: 0,
                    index: 9,
                },
                MapTile {
                    type_index: 1,
                    index: 10,
                },
                MapTile {
                    type_index: 1,
                    index: 11,
                },
                MapTile {
                    type_index: 0,
                    index: 12,
                },
                MapTile {
                    type_index: 0,
                    index: 13,
                },
                MapTile {
                    type_index: 0,
                    index: 14,
                },
                MapTile {
                    type_index: 0,
                    index: 15,
                },
                MapTile {
                    type_index: 1,
                    index: 16,
                },
                MapTile {
                    type_index: 1,
                    index: 17,
                },
                MapTile {
                    type_index: 0,
                    index: 18,
                },
                MapTile {
                    type_index: 0,
                    index: 19,
                },
                MapTile {
                    type_index: 0,
                    index: 20,
                },
                MapTile {
                    type_index: 0,
                    index: 21,
                },
                MapTile {
                    type_index: 1,
                    index: 22,
                },
                MapTile {
                    type_index: 1,
                    index: 23,
                },
                MapTile {
                    type_index: 1,
                    index: 24,
                },
            ],
            enemy_types: vec![],
            enemies: vec![],
            portals: vec![
                Portal {
                    index: 10,
                    target: Some(1),
                    orientation: PortalOrientation::Right,
                },
                Portal {
                    index: 24,
                    target: Some(3),
                    orientation: PortalOrientation::Left,
                },
            ],
            powerup_types: vec![SPEED_POWERUP_TYPE],
            powerups: vec![Powerup {
                type_index: 0,
                index: 17,
            }],
            char_start_index: 10,
        }),
        3 => Ok(LevelResponse {
            tile_types: vec![DIRT_MAP_TILE, LADDER_MAP_TILE, WALL_MAP_TILE],
            tiles: vec![
                MapTile {
                    type_index: 1,
                    index: 0,
                },
                MapTile {
                    type_index: 2,
                    index: 1,
                },
                MapTile {
                    type_index: 2,
                    index: 2,
                },
                MapTile {
                    type_index: 2,
                    index: 3,
                },
                MapTile {
                    type_index: 2,
                    index: 4,
                },
                MapTile {
                    type_index: 1,
                    index: 5,
                },
                MapTile {
                    type_index: 0,
                    index: 6,
                },
                MapTile {
                    type_index: 0,
                    index: 7,
                },
                MapTile {
                    type_index: 0,
                    index: 8,
                },
                MapTile {
                    type_index: 2,
                    index: 9,
                },
                MapTile {
                    type_index: 1,
                    index: 10,
                },
                MapTile {
                    type_index: 0,
                    index: 11,
                },
                MapTile {
                    type_index: 2,
                    index: 12,
                },
                MapTile {
                    type_index: 0,
                    index: 13,
                },
                MapTile {
                    type_index: 1,
                    index: 14,
                },
                MapTile {
                    type_index: 2,
                    index: 15,
                },
                MapTile {
                    type_index: 0,
                    index: 16,
                },
                MapTile {
                    type_index: 0,
                    index: 17,
                },
                MapTile {
                    type_index: 0,
                    index: 18,
                },
                MapTile {
                    type_index: 1,
                    index: 19,
                },
                MapTile {
                    type_index: 2,
                    index: 20,
                },
                MapTile {
                    type_index: 2,
                    index: 21,
                },
                MapTile {
                    type_index: 2,
                    index: 22,
                },
                MapTile {
                    type_index: 2,
                    index: 23,
                },
                MapTile {
                    type_index: 1,
                    index: 24,
                },
            ],
            enemy_types: vec![CRAWLER_ENEMY_TYPE, RUNNER_ENEMY_TYPE],
            enemies: vec![
                Enemy {
                    id: 0,
                    type_index: 0,
                    path: vec![6, 11, 16, 17, 18, 13, 8, 7],
                },
                Enemy {
                    id: 1,
                    type_index: 1,
                    path: vec![18, 13, 8, 7, 6, 11, 16, 17],
                },
            ],
            portals: vec![
                Portal {
                    index: 0,
                    target: Some(2),
                    orientation: PortalOrientation::Down,
                },
                Portal {
                    index: 24,
                    target: None,
                    orientation: PortalOrientation::Up,
                },
            ],
            powerup_types: vec![],
            powerups: vec![],
            char_start_index: 0,
        }),
        _ => {
            let error = format!("Unknown level: {}", level);
            error!("{} - request {}", error, context.aws_request_id);
            Err(context.new_error(error.as_str()))
        }
    }
}
