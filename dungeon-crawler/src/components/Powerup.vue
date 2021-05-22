<template>
  <div>
    <img
      :src="require(`../../../../assets/Powerup-${powerup.type}.png`)"
      :style="powerupStyle"
    />
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent, PropType } from 'vue'
import { PowerupData } from '../model/Model'

export default defineComponent({
  name: 'Powerup',
  props: {
    powerup: {
      type: Object as PropType<PowerupData>,
      required: true
    }
  },
  computed: {
    powerupStyle (): CSSProperties {
      const x = ((this.powerup.index % 5) * 200) + 45
      const y = Math.floor(this.powerup.index / 5) * 200 + 45
      return {
        position: 'fixed',
        left: x + 'px',
        top: y + 'px',
        zIndex: 20
      }
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
