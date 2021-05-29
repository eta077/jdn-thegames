<template>
  <MapGrid :mapTiles="this.mapTiles" :enemies="this.enemies" :powerups="this.powerups" :portals="this.portals"
    :character="this.character" @enemyMoved="this.handleEnemyMoved"/>
  <AudioControl />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import AudioControl from './components/AudioControl.vue'
import MapGrid from './components/MapGrid.vue'
import { EnemyMovedEvent } from './model/Events'
import { CharacterData, EnemyData, LevelRequest, LevelResponse, MapTileData, Orientation, PortalInfo, PowerupData } from './model/Model'

export default defineComponent({
  name: 'App',
  components: {
    AudioControl,
    MapGrid
  },
  data () {
    return {
      mapTiles: [] as MapTileData[],
      enemies: [] as EnemyData[],
      powerups: [] as PowerupData[],
      portals: [] as PortalInfo[],
      character: { startIndex: 0, curIndex: 0, orientation: Orientation.Right, step: false, curHealth: 1, powerup: '' } as CharacterData
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
          const resPowerups: PowerupData[] = []
          for (const powerup of data.powerups) {
            const powerupType = data.powerupTypes[powerup.typeIndex]
            resPowerups.push({ type: powerupType.name, healthBoost: powerupType.healthBoost, speedBoost: powerupType.speedBoost, index: powerup.index })
          }
          this.enemies = resEnemies
          this.powerups = resPowerups
          this.portals = data.portals
          this.character.startIndex = data.charStartIndex
          this.character.curIndex = data.charStartIndex
          this.character.orientation = Orientation.Right
        })
    },
    handleKeypress (e: KeyboardEvent) {
      switch (e.code) {
        case 'KeyW': {
          if (!this.checkCharacterCurIndex(Orientation.Down)) {
            return
          }
          const newIndex = this.character.curIndex - 5
          const newY = newIndex / 5
          if (newY < 0) {
            return
          }
          if (this.checkCharacterNewIndex(newIndex)) {
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyS': {
          if (!this.checkCharacterCurIndex(Orientation.Up)) {
            return
          }
          const newIndex = this.character.curIndex + 5
          const newY = newIndex / 5
          if (newY >= 5) {
            return
          }
          if (this.checkCharacterNewIndex(newIndex)) {
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyA': {
          if (!this.checkCharacterCurIndex(Orientation.Right)) {
            return
          }
          const newX = (this.character.curIndex % 5) - 1
          if (newX < 0) {
            return
          }
          const newIndex = this.character.curIndex - 1
          if (this.checkCharacterNewIndex(newIndex)) {
            this.character.orientation = Orientation.Left
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyD': {
          if (!this.checkCharacterCurIndex(Orientation.Left)) {
            return
          }
          const newX = (this.character.curIndex % 5) + 1
          if (newX === 5) {
            return
          }
          const newIndex = this.character.curIndex + 1
          if (this.checkCharacterNewIndex(newIndex)) {
            this.character.orientation = Orientation.Right
            this.character.curIndex = newIndex
          }
          break
        }
        case 'KeyJ': {
          this.startCharacterJump()
          break
        }
      }
    },
    checkCharacterCurIndex (orientation: string): boolean {
      for (const portal of this.portals) {
        if (portal.index === this.character.curIndex && portal.orientation === orientation && portal.target != null) {
          this.requestLevel(portal.target)
          return false
        }
      }
      return true
    },
    checkCharacterNewIndex (newIndex: number): boolean {
      if (this.mapTiles[newIndex].type.impenetrable) {
        return false
      }
      for (const enemy of this.enemies) {
        if (enemy.curIndex === newIndex) {
          this.character.curIndex = this.character.startIndex
          return false
        }
      }
      let removedPowerup = this.powerups.length
      for (let i = 0; i < this.powerups.length; i++) {
        const powerup = this.powerups[i]
        if (powerup.index === newIndex) {
          this.character.powerup = powerup.type
          removedPowerup = i
        }
      }
      this.powerups.splice(removedPowerup, 1)
      this.character.step = !this.character.step
      return true
    },
    startCharacterJump () {
      if (!this.character.jumping) {
        this.character.jumping = true
        setTimeout(this.endCharacterJump, 250)
      }
    },
    endCharacterJump () {
      this.character.jumping = false
      for (let i = 0; i < this.enemies.length; i++) {
        const enemy = this.enemies[i]
        if (enemy.curIndex === this.character.curIndex) {
          enemy.curHealth -= 1
          if (enemy.curHealth >= 0) {
            setTimeout(this.startCharacterJump, 250)
          }
          break
        }
      }
    },
    handleEnemyMoved (event: EnemyMovedEvent) {
      const enemyIndex = event.enemyId
      const enemy = this.enemies[enemyIndex]
      if (enemy.curHealth <= 0) {
        this.enemies.splice(enemyIndex, 1)
        for (let i = enemyIndex; i < this.enemies.length; i++) {
          this.enemies[i].id -= 1
        }
      }
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
