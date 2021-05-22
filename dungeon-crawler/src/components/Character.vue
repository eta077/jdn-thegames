<template>
  <div>
    <img
      :src="require(`../../../../assets/${characterImage}.png`)"
      :style="characterStyle"
    />
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent, PropType } from 'vue'
import { CharacterData } from '../model/Model'

export default defineComponent({
  name: 'Character',
  props: {
    character: {
      type: Object as PropType<CharacterData>,
      required: true
    }
  },
  computed: {
    characterImage (): string {
      let step
      if (this.character.step) {
        step = '-step'
      } else {
        step = ''
      }
      let powerup
      if (this.character.powerup === '') {
        powerup = ''
      } else {
        powerup = this.character.powerup + '-'
      }
      return 'character-' + powerup + this.character.orientation + step
    },
    characterStyle (): CSSProperties {
      const x = ((this.character.curIndex % 5) * 200) + 50
      let y = Math.floor(this.character.curIndex / 5) * 200 + 50
      if (this.character.jumping) {
        y -= 100
      }
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
