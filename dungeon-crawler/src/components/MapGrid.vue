<template>
  <div>
    <div v-for="tile in mapTiles" :key="tile.index">
      <MapTile :tile="tile"/>
    </div>
    <div v-for="enemy in enemies" :key="enemy.id">
      <Enemy :enemy="enemy" v-bind="$attrs"/>
    </div>
    <div v-for="powerup in powerups" :key="powerup.index">
      <Powerup :powerup="powerup"/>
    </div>
    <div v-for="portal in portals" :key="portal.index">
      <Portal :portal="portal"/>
    </div>
    <Character :character="character"/>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue'
import MapTile from './MapTile.vue'
import Enemy from './Enemy.vue'
import Portal from './Portal.vue'
import Powerup from './Powerup.vue'
import Character from './Character.vue'
import { CharacterData, EnemyInfo, MapTileData, PortalInfo, PowerupData } from '../model/Model'

export default defineComponent({
  name: 'MapGrid',
  components: { MapTile, Enemy, Portal, Powerup, Character },
  props: {
    mapTiles: {
      type: Array as PropType<Array<MapTileData>>,
      required: true
    },
    enemies: {
      type: Array as PropType<Array<EnemyInfo>>,
      required: true
    },
    portals: {
      type: Array as PropType<Array<PortalInfo>>,
      required: true
    },
    powerups: {
      type: Array as PropType<Array<PowerupData>>,
      required: true
    },
    character: {
      type: Object as PropType<CharacterData>,
      required: true
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
