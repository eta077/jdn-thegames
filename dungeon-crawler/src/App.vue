<template>
  <MapGrid :mapTiles="this.mapTiles" :enemies="this.enemies" :portals="this.portals"
    :character="this.character" @enemyMoved="this.handleEnemyMoved"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import MapGrid from './components/MapGrid.vue'
import { EnemyMovedEvent } from './model/Events'
import { CharacterData, EnemyData, LevelRequest, LevelResponse, MapTileData, PortalInfo } from './model/Model'

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
      character: { curIndex: 0, curHealth: 1 } as CharacterData
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
            const enemyType = data.enemyTypes[enemy.typeIndex]
            resEnemies.push({ id: enemy.id, type: enemyType, path: enemy.path, curIndex: enemy.path[0], curHealth: enemyType.health })
          }
          this.enemies = resEnemies
          this.portals = data.portals
          this.character.curIndex = data.charStartIndex
        })
    },
    handleKeypress (e: KeyboardEvent) {
      switch (e.code) {
        case 'KeyW': {
          const newIndex = this.character.curIndex - 5
          const newY = newIndex / 5
          if (newY < 0) {
            return
          }
          if (this.moveCharacter(newIndex)) {
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyS': {
          const newIndex = this.character.curIndex + 5
          const newY = newIndex / 5
          if (newY === 5) {
            return
          }
          if (this.moveCharacter(newIndex)) {
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyA': {
          const newX = (this.character.curIndex % 5) - 1
          if (newX < 0) {
            return
          }
          const newIndex = this.character.curIndex - 1
          if (this.moveCharacter(newIndex)) {
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyD': {
          const newX = (this.character.curIndex % 5) + 1
          if (newX === 5) {
            return
          }
          const newIndex = this.character.curIndex + 1
          if (this.moveCharacter(newIndex)) {
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyJ': {
          if (!this.character.jumping) {
            this.character.jumping = true
            setTimeout(this.endCharacterJump, 250)
          }
          break
        }
      }
    },
    moveCharacter (newIndex: number): boolean {
      if (this.mapTiles[newIndex].type.impenetrable) {
        return false
      }
      for (const enemy of this.enemies) {
        if (enemy.curIndex === newIndex) {
          this.character.curIndex = 5
          return false
        }
      }
      for (const portal of this.portals) {
        if (portal.index === newIndex && portal.target != null) {
          this.requestLevel(portal.target)
          return false
        }
      }
      return true
    },
    endCharacterJump () {
      this.character.jumping = false
      let removedEnemy = this.enemies.length
      for (let i = 0; i < this.enemies.length; i++) {
        if (this.enemies[i].curIndex === this.character.curIndex) {
          removedEnemy = i
          break
        }
      }
      if (removedEnemy < this.enemies.length) {
        this.enemies.splice(removedEnemy, 1)
      }
    },
    handleEnemyMoved (event: EnemyMovedEvent) {
      this.enemies[event.enemyId].curIndex = event.newIndex
      if (!this.character.jumping && event.newIndex === this.character.curIndex) {
        this.character.curIndex = 5
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
