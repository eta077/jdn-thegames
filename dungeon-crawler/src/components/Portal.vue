<template>
  <div>
    <img
      :src="require(`../../../../assets/Portal-${portal.orientation}.png`)"
      :style="portalStyle"
    />
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent, PropType } from 'vue'
import { PortalInfo, Orientation } from '../model/Model'

export default defineComponent({
  name: 'Portal',
  props: {
    portal: {
      type: Object as PropType<PortalInfo>,
      required: true
    }
  },
  computed: {
    portalStyle (): CSSProperties {
      let x = ((this.portal.index % 5) * 200)
      if (this.portal.orientation === Orientation.Right) {
        x -= 100
      } else if (this.portal.orientation === Orientation.Left) {
        x += 100
      }
      let y = Math.floor(this.portal.index / 5) * 200
      if (this.portal.orientation === Orientation.Down) {
        y -= 100
      } else if (this.portal.orientation === Orientation.Up) {
        y += 100
      }
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
