<template>
  <div>
    <img
      :src="require(`../../../../assets/${tile.type.name}.png`)"
      :style="tileStyle"
    />
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent, PropType } from 'vue'
import { MapTileInfo } from '../model/Model'

export default defineComponent({
  name: 'MapTile',
  props: {
    tile: {
      type: Object as PropType<MapTileInfo>,
      required: true
    }
  },
  computed: {
    tileStyle (): CSSProperties {
      const x = ((this.tile.index % 5) * 200)
      const y = Math.floor(this.tile.index / 5) * 200
      return {
        position: 'fixed',
        left: x + 'px',
        top: y + 'px'
      }
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
