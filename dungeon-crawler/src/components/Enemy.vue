<template>
  <div>
    <img
      :src="require(`../../../../assets/${enemy.type}.png`)"
      :style="enemyStyle"
    />
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent } from 'vue'
import { EnemyInfo } from '../model/Model'

export default defineComponent({
  name: 'Enemy',
  data () {
    return {
      reverse: false,
      index: 0
    }
  },
  props: {
    enemy: {
      type: EnemyInfo,
      required: true
    }
  },
  computed: {
    enemyStyle (): CSSProperties {
      const x = ((this.enemy.path[this.index] % 5) * 200)
      const y = Math.floor(this.enemy.path[this.index] / 5) * 200
      return {
        position: 'fixed',
        left: x + 'px',
        top: y + 'px'
      }
    }
  },
  methods: {
    incrementIndex () {
      if (this.reverse) {
        if (this.index === 1) {
          this.reverse = false
        }
        this.index -= 1
      } else {
        if (this.index === this.enemy.path.length - 2) {
          this.reverse = true
        }
        this.index += 1
      }
    }
  },
  mounted () {
    setInterval(this.incrementIndex, 1000)
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
