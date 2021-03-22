<template>
  <div>
    <img
      :src="require(`../../../../assets/Pipe-${portal.orientation}.png`)"
      :style="portalStyle"
    />
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent } from 'vue'
import { PortalInfo } from '../model/Model'

export default defineComponent({
  name: 'Portal',
  props: {
    portal: {
      type: PortalInfo,
      required: true
    }
  },
  computed: {
    portalStyle (): CSSProperties {
      const x = ((this.portal.index % 5) * 200)
      const y = Math.floor(this.portal.index / 5) * 200
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
