<template>
  <MapGrid :mapTiles="this.mapTiles" :portals="this.portals" :charX="this.charX" :charY="this.charY"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import MapGrid from './components/MapGrid.vue'
import { LevelResponse, MapTileInfo, PortalInfo } from './model/Model'

export default defineComponent({
  name: 'App',
  components: {
    MapGrid
  },
  data () {
    return {
      mapTiles: [] as MapTileInfo[],
      portals: [] as PortalInfo[],
      charX: 0,
      charY: 0
    }
  },
  methods: {
    requestLevel (level: number) {
      const requestOptions: RequestInit = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ level: level }),
        redirect: 'follow'
      }
      fetch('https://mdnouzkj44.execute-api.us-west-2.amazonaws.com/prod', requestOptions)
        .then(response => response.json() as Promise<LevelResponse>)
        .then(data => {
          this.mapTiles = data.tiles
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
          for (const portal of this.portals) {
            if (portal.index === newIndex && portal.target != null) {
              this.requestLevel(portal.target)
              return
            }
          }
          this.charY -= 1
          break
        }
        case 'KeyS': {
          this.charY += 1
          break
        }
        case 'KeyA': {
          const newX = this.charX - 1
          if (newX < 0) {
            return
          }
          const newIndex = this.charY * 5 + newX % 5
          for (const portal of this.portals) {
            if (portal.index === newIndex && portal.target != null) {
              this.requestLevel(portal.target)
              return
            }
          }
          this.charX -= 1
          break
        }
        case 'KeyD': {
          const newX = this.charX + 1
          if (newX === 5) {
            return
          }
          const newIndex = this.charY * 5 + newX % 5
          for (const portal of this.portals) {
            if (portal.index === newIndex && portal.target != null) {
              this.requestLevel(portal.target)
              return
            }
          }
          this.charX += 1
          break
        }
      }
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
