<template>
  <div class="page">
    <div class="toolbar">
      <input v-model="sourceUrl" placeholder="源地址（可空）" class="input" />
      <input v-model="stream" placeholder="stream" class="input small" />
      <button @click="createProxyAndPlay" :disabled="loading">创建代理并播放</button>
      <button @click="stop" :disabled="!player">停止</button>
    </div>

    <div class="grid">
      <div :id="playerId"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from "vue"
import axios from "axios"
import Player from "xgplayer"
import "xgplayer/dist/index.min.css"
import FlvPlugin from "xgplayer-flv"

const apiBase = ref("http://127.0.0.1:8080")
const playerId = "mse1"

const sourceUrl = ref("")
const stream = ref("test")
const app = ref("live")
const vhost = ref("__defaultVhost__")
const secret = ref("123456")

const loading = ref(false)
const player = ref<Player | null>(null)

const playUrl = computed(() => {
  return `http://127.0.0.1:8888/${app.value}/${stream.value}.live.flv`
})

const stop = () => {
  player.value?.destroy()
  player.value = null
}

const createProxyAndPlay = async () => {
  loading.value = true
  try {
    await axios.get(`${apiBase.value}/index/api/addStreamProxy`, {
      params: {
        url: sourceUrl.value,
        stream: stream.value,
        app: app.value,
        vhost: vhost.value,
        secret: secret.value,
        enable_hls: 0,
        enable_mp4: 0,
        enable_rtsp: 0,
        enable_rtmp: 0,
      },
    })

    stop()
    player.value = new Player({
      id: playerId,
      url: playUrl.value,
      isLive: true,
      height: "200px",
      width: "200px",
      plugins: [FlvPlugin],
    })
  } finally {
    loading.value = false
  }
}

onBeforeUnmount(() => {
  stop()
})
</script>

<style scoped>
.page {
  padding: 12px;
}

.toolbar {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 12px;
}

.input {
  height: 32px;
  padding: 0 10px;
  border: 1px solid #ccc;
  border-radius: 6px;
  min-width: 280px;
}

.input.small {
  min-width: 120px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 200px));
  gap: 8px;
}
</style>

