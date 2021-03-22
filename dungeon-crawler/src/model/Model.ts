
export enum MapTileType {
  Dirt,
  Bushes,
}

export class MapTileInfo {
  index = 0;
  type = MapTileType.Bushes;

  constructor (index: number, type: MapTileType) {
    this.index = index
    this.type = type
  }

  isImpenetrable (): boolean {
    return this.type === MapTileType.Bushes
  }
}

export enum PortalOrientation {
  Up,
  Down,
  Left,
  Right,
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

export class LevelRequest {
  level = 0;

  constructor (level: number) {
    this.level = level
  }
}

export class LevelResponse {
  char_start_index = 0;
  tiles: MapTileInfo[] = [];
  portals: PortalInfo[] = [];

  constructor (char_start_index: number, tiles: MapTileInfo[], portals: PortalInfo[]) {
    this.char_start_index = char_start_index
    this.tiles = tiles
    this.portals = portals
  }
}
