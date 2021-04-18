<template>
  <div>
    <audio style="display:none" ref='audio' loop>
        <source :src="require(`../../../../assets/music.mp3`)" />
    </audio>
    <img
      @click="handleClick"
      :src="require(`../../../../assets/${imageName}.png`)"
      :style="{
        position: 'fixed',
        left: '0px',
        top: '1000px',
        zIndex: 100
      }"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'AudioControl',
  data () {
    return {
      muted: true
    }
  },
  computed: {
    imageName (): string {
      if (this.muted) {
        return 'audio-muted'
      } else {
        return 'audio'
      }
    }
  },
  methods: {
    handleClick () {
      const audio = this.$refs.audio as HTMLAudioElement
      if (this.muted) {
        audio.play()
        this.muted = false
      } else {
        audio.pause()
        audio.currentTime = 0
        this.muted = true
      }
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
