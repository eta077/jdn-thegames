
export enum MapTileType {
  Dirt = 'Dirt',
  Bushes = 'Bushes',
}

export class MapTileInfo {
  index = 0;
  type = MapTileType.Bushes;

  constructor (index: number, type: MapTileType) {
    this.index = index
    this.type = type
  }
}

export function isTileImpenetrable (tile: MapTileInfo): boolean {
  console.log(MapTileType.Bushes)
  return tile.type === MapTileType.Bushes
}

export enum PortalOrientation {
  Up = 'Up',
  Down = 'Down',
  Left = 'Left',
  Right = 'Right',
}

export class PortalInfo {
  index = 0;
  target = 0;
  orientation = PortalOrientation.Right;

  constructor (index: number, target: number, orientation: PortalOrientation) {
    this.index = index
    this.target = target
    this.orientation = orientation
  }
}

export enum EnemyType {
  Crawler = 'Crawler',
  Walker = 'Walker',
}

export class EnemyInfo {
  type = EnemyType.Crawler;
  path: number[] = [];

  constructor (type: EnemyType, path: number[]) {
    this.type = type
    this.path = path
  }
}

export class LevelRequest {
  level = 0;

  constructor (level: number) {
    this.level = level
  }
}

export class LevelResponse {
  charStartIndex = 0;
  tiles: MapTileInfo[] = [];
  enemies: EnemyInfo[] = [];
  portals: PortalInfo[] = [];

  constructor (charStartIndex: number, tiles: MapTileInfo[], enemies: EnemyInfo[], portals: PortalInfo[]) {
    this.charStartIndex = charStartIndex
    this.tiles = tiles
    this.enemies = enemies
    this.portals = portals
  }
}
