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

fn level_handler(e: LevelRequest, context: Context) -> Result<LevelResponse, HandlerError> {
    let level = e.level;
    match level {
        1 => Ok(LevelResponse {
            char_start_index: 5,
            tiles: vec![
                MapTile {
                    index: 0,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 1,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 2,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 3,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 4,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 5,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 6,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 7,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 8,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 9,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 10,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 11,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 12,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 13,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 14,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 15,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 16,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 17,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 18,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 19,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 20,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 21,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 22,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 23,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 24,
                    r#type: MapTileType::Bushes,
                },
            ],
            enemies: vec![
                Enemy {
                    r#type: EnemyType::Walker,
                    path: vec![19, 18, 17, 12, 7],
                },
            ],
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
        }),
        2 => Ok(LevelResponse {
            char_start_index: 10,
            tiles: vec![
                MapTile {
                    index: 0,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 1,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 2,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 3,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 4,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 5,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 6,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 7,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 8,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 9,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 10,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 11,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 12,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 13,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 14,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 15,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 16,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 17,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 18,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 19,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 20,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 21,
                    r#type: MapTileType::Bushes,
                },
                MapTile {
                    index: 22,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 23,
                    r#type: MapTileType::Dirt,
                },
                MapTile {
                    index: 24,
                    r#type: MapTileType::Dirt,
                },
            ],
            enemies: vec![],
            portals: vec![
                Portal {
                    index: 10,
                    target: Some(1),
                    orientation: PortalOrientation::Right,
                },
                Portal {
                    index: 24,
                    target: None,
                    orientation: PortalOrientation::Left,
                },
            ],
        }),
        _ => {
            let error = format!("Unknown level: {}", level);
            error!("{} - request {}", error, context.aws_request_id);
            Err(context.new_error(error.as_str()))
        }
    }
}
