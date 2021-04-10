<template>
  <div>
    <img src="../../../../assets/character.png" :style="charStyle" />
    <div v-for="tile in mapTiles" :key="tile.index">
      <MapTile :tile="tile"/>
    </div>
    <div v-for="enemy in enemies" :key="enemy.path[0]">
      <Enemy :enemy="enemy"/>
    </div>
    <div v-for="portal in portals" :key="portal.index">
      <Portal :portal="portal"/>
    </div>
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent, PropType } from 'vue'
import MapTile from './MapTile.vue'
import Portal from './Portal.vue'
import Enemy from './Enemy.vue'
import { EnemyInfo, MapTileInfo, PortalInfo } from '../model/Model'

export default defineComponent({
  name: 'MapGrid',
  components: { MapTile, Enemy, Portal },
  props: {
    mapTiles: {
      type: Array as PropType<Array<MapTileInfo>>,
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
    charX: {
      type: Number,
      required: true
    },
    charY: {
      type: Number,
      required: true
    }
  },
  computed: {
    charStyle (): CSSProperties {
      const x = (this.charX * 200) + 50
      const y = (this.charY * 200) + 50
      return {
        position: 'fixed',
        left: x + 'px',
        top: y + 'px',
        zIndex: 10
      }
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
