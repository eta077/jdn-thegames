<template>
  <div>
    <img
      :src="require(`../../../../assets/${enemy.type.name}.png`)"
      :style="enemyStyle"
    />
  </div>
</template>

<script lang="ts">
import { CSSProperties, defineComponent, PropType } from 'vue'
import { EnemyData } from '../model/Model'

export default defineComponent({
  name: 'Enemy',
  data () {
    return {
      reverse: false,
      curIndex: 0,
      intervalId: 0
    }
  },
  props: {
    enemy: {
      type: Object as PropType<EnemyData>,
      required: true
    }
  },
  computed: {
    enemyStyle (): CSSProperties {
      const x = ((this.enemy.path[this.curIndex] % 5) * 200)
      const y = Math.floor(this.enemy.path[this.curIndex] / 5) * 200
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
        if (this.curIndex === 1) {
          this.reverse = false
        }
        this.curIndex -= 1
      } else {
        if (this.curIndex === this.enemy.path.length - 2) {
          this.reverse = true
        }
        this.curIndex += 1
      }
      this.$emit('enemyMoved', { enemyId: this.enemy.id, newIndex: this.enemy.path[this.curIndex] })
    }
  },
  emits: ['enemyMoved'],
  mounted () {
    this.intervalId = setInterval(this.incrementIndex, this.enemy.type.speed * 1000)
  },
  unmounted () {
    clearInterval(this.intervalId)
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
