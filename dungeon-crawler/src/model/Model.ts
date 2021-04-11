
export interface MapTileType {
  name: string;
  impenetrable: boolean;
}

export interface MapTileData {
  type: MapTileType;
  index: number;
}

export interface MapTileInfo {
  typeIndex: number;
  index: number;
}

export interface PortalInfo {
  index: number;
  target: number;
  orientation: string;
}

export interface EnemyType {
  name: string;
  health: number;
  speed: number;
}

export interface EnemyData {
  type: EnemyType;
  path: number[];
}

export interface EnemyInfo {
  typeIndex: number;
  path: number[];
}

export interface LevelResponse {
  charStartIndex: number;
  tileTypes: MapTileType[];
  tiles: MapTileInfo[];
  enemyTypes: EnemyType[];
  enemies: EnemyInfo[];
  portals: PortalInfo[];
}

export class LevelRequest {
  level = 0;

  constructor (level: number) {
    this.level = level
  }
}
