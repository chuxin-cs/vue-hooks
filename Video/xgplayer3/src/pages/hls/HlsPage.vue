<template>
  <div class="page">
    <div class="toolbar">
      <button @click="startAll" :disabled="isRunning">播放</button>
      <button @click="stopAll" :disabled="!isRunning">停止</button>
    </div>

    <div class="grid">
      <div v-for="id in ids" :key="id" :id="id"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from "vue"
import Player from "xgplayer"
import "xgplayer/dist/index.min.css"
import FlvPlugin from "xgplayer-flv"
import HlsPlugin from "xgplayer-hls"

const ids = Array.from({ length: 21 }, (_, i) => `mse${i + 1}`)

const url = ref("http://127.0.0.1:8888/live/test/index.m3u8")

const players = ref<Player[]>([])
const isRunning = computed(() => players.value.length > 0)

const createPlayer = (id: string) => {
  return new Player({
    id,
    url: url.value,
    height: "200px",
    width: "200px",
    plugins: [FlvPlugin, HlsPlugin],
    autoplay: true,
    muted: true,
    hls: {
      liveSyncDuration: 1,
      liveMaxLatencyDuration: 2,
      maxBufferLength: 2,
      maxMaxBufferLength: 4,
      backBufferLength: 0,
      lowLatencyMode: true,
      enableWorker: true,
      fragLoadingTimeOut: 20000,
      manifestLoadingTimeOut: 10000,
      fragLoadingRetryDelay: 1000,
      manifestLoadingRetryDelay: 1000,
      appendErrorMaxRetry: 3,
    },
    preloadTime: 1,
  })
}

const stopAll = () => {
  const current = players.value
  players.value = []
  current.forEach((p) => {
    p.destroy()
  })
}

const startAll = () => {
  stopAll()
  players.value = ids.map((id) => createPlayer(id))
}

onBeforeUnmount(() => {
  stopAll()
})
</script>

<style scoped>
.page {
  padding: 12px;
}

.toolbar {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 200px));
  gap: 8px;
}
</style>

