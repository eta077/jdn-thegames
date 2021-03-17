<template>
  <div>
    <img src="../../../../assets/character.png" :style="charStyle" />
    <div v-for="tile in mapTiles" :key="tile.index">
      <MapTile :tile="tile"/>
    </div>
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent } from 'vue'
import MapTile from './MapTile.vue'
import MapTileInfo from '../model/MapTile'

export default defineComponent({
  name: 'MapGrid',
  components: { MapTile },
  data () {
    return {
      mapTiles: [
        new MapTileInfo('bushes', 0),
        new MapTileInfo('bushes', 1),
        new MapTileInfo('bushes', 2),
        new MapTileInfo('bushes', 3),
        new MapTileInfo('bushes', 4),
        new MapTileInfo('dirt', 5),
        new MapTileInfo('dirt', 6),
        new MapTileInfo('dirt', 7),
        new MapTileInfo('bushes', 8),
        new MapTileInfo('bushes', 9),
        new MapTileInfo('bushes', 10),
        new MapTileInfo('bushes', 11),
        new MapTileInfo('dirt', 12),
        new MapTileInfo('bushes', 13),
        new MapTileInfo('bushes', 14),
        new MapTileInfo('bushes', 15),
        new MapTileInfo('bushes', 16),
        new MapTileInfo('dirt', 17),
        new MapTileInfo('dirt', 18),
        new MapTileInfo('dirt', 19),
        new MapTileInfo('bushes', 20),
        new MapTileInfo('bushes', 21),
        new MapTileInfo('bushes', 22),
        new MapTileInfo('bushes', 23),
        new MapTileInfo('bushes', 24)
      ],
      charX: 0,
      charY: 0
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
        zIndex: 100
      }
    }
  },
  methods: {
    handleKeypress (e: KeyboardEvent) {
      switch (e.code) {
        case 'KeyW':
          this.charY -= 1
          break
        case 'KeyS':
          this.charY += 1
          break
        case 'KeyA':
          this.charX -= 1
          break
        case 'KeyD':
          this.charX += 1
          break
      }
    }
  },
  mounted () {
    window.addEventListener('keypress', this.handleKeypress)
  },
  unmounted () {
    window.removeEventListener('keypress', this.handleKeypress)
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
