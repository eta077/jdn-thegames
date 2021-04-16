
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
  id: number;
  type: EnemyType;
  path: number[];
  curIndex: number;
  curHealth: number;
}

export interface EnemyInfo {
  id: number;
  typeIndex: number;
  path: number[];
}

export interface CharacterData {
  curIndex: number;
  curHealth: number;
  jumping: boolean;
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
