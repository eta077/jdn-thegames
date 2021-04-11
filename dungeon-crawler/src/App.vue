<template>
  <MapGrid :mapTiles="this.mapTiles" :enemies="this.enemies" :portals="this.portals"
    :charX="this.charX" :charY="this.charY"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import MapGrid from './components/MapGrid.vue'
import { EnemyData, LevelRequest, LevelResponse, MapTileData, PortalInfo } from './model/Model'

export default defineComponent({
  name: 'App',
  components: {
    MapGrid
  },
  data () {
    return {
      mapTiles: [] as MapTileData[],
      enemies: [] as EnemyData[],
      portals: [] as PortalInfo[],
      charX: 0,
      charY: 0
    }
  },
  methods: {
    requestLevel (level: number) {
      const levelRequest = new LevelRequest(level)
      const requestOptions: RequestInit = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(levelRequest),
        redirect: 'follow'
      }
      fetch('https://mdnouzkj44.execute-api.us-west-2.amazonaws.com/prod', requestOptions)
        .then(response => response.json() as Promise<LevelResponse>)
        .then(data => {
          const resTiles: MapTileData[] = []
          for (const tile of data.tiles) {
            resTiles.push({ type: data.tileTypes[tile.typeIndex], index: tile.index })
          }
          this.mapTiles = resTiles
          const resEnemies: EnemyData[] = []
          for (const enemy of data.enemies) {
            resEnemies.push({ type: data.enemyTypes[enemy.typeIndex], path: enemy.path })
          }
          this.enemies = resEnemies
          this.portals = data.portals
          this.charX = data.charStartIndex % 5
          this.charY = data.charStartIndex / 5
        })
    },
    handleKeypress (e: KeyboardEvent) {
      switch (e.code) {
        case 'KeyW': {
          const newY = this.charY - 1
          if (newY < 0) {
            return
          }
          const newIndex = newY * 5 + this.charX % 5
          if (this.moveCharacter(newIndex)) {
            this.charY -= 1
          }
          break
        }
        case 'KeyS': {
          const newY = this.charY + 1
          if (newY === 5) {
            return
          }
          const newIndex = newY * 5 + this.charX % 5
          if (this.moveCharacter(newIndex)) {
            this.charY += 1
          }
          break
        }
        case 'KeyA': {
          const newX = this.charX - 1
          if (newX < 0) {
            return
          }
          const newIndex = this.charY * 5 + newX % 5
          if (this.moveCharacter(newIndex)) {
            this.charX -= 1
          }
          break
        }
        case 'KeyD': {
          const newX = this.charX + 1
          if (newX === 5) {
            return
          }
          const newIndex = this.charY * 5 + newX % 5
          if (this.moveCharacter(newIndex)) {
            this.charX += 1
          }
          break
        }
      }
    },
    moveCharacter (newIndex: number): boolean {
      if (this.mapTiles[newIndex].type.impenetrable) {
        return false
      }
      for (const portal of this.portals) {
        if (portal.index === newIndex && portal.target != null) {
          this.requestLevel(portal.target)
          return false
        }
      }
      return true
    }
  },
  mounted () {
    this.requestLevel(1)
    window.addEventListener('keypress', this.handleKeypress)
  },
  unmounted () {
    window.removeEventListener('keypress', this.handleKeypress)
  }
})
</script>

<style>
</style>
