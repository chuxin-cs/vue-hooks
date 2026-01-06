<template>
  <div class="page">
    <div class="toolbar">
      <input v-model="url" class="input" placeholder="播放地址" />
      <button @click="start" :disabled="loading">播放</button>
      <button @click="stop" :disabled="!player">停止</button>
    </div>

    <div class="grid">
      <div v-for="id in ids" :key="id" :id="id"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue"

const ids = Array.from({ length: 9 }, (_, i) => `app${i + 1}`)

const url = ref("http://127.0.0.1:8888/live/test/index.m3u8")
const loading = ref(false)
const player = ref<any>(null)

const getJessibuca = () => (window as any)?.Jessibuca

const waitJessibuca = async () => {
  const deadline = Date.now() + 8000
  while (Date.now() < deadline) {
    if (getJessibuca()) return
    await new Promise((r) => setTimeout(r, 100))
  }
}

const stop = () => {
  const p = player.value
  player.value = null
  p?.destroy?.()
  p?.close?.()
  p?.pause?.()
}

const start = async () => {
  loading.value = true
  try {
    await waitJessibuca()
    const Jessibuca = getJessibuca()
    if (!Jessibuca) return

    stop()
    const container = document.getElementById("app1")
    player.value = new Jessibuca({
      container,
      isFlv: true,
      videoBuffer: 0.2,
      isResize: false,
      hasAudio: true,
      loadingText: "加载中...",
      showBandwidth: false,
      loadingTimeout: 10,
      heartTimeout: 10,
      heartTimeoutReplay: false,
      loadingTimeoutReplay: false,
      supportDblclickFullscreen: true,
      operateBtns: {
        fullscreen: true,
        screenshot: true,
        play: true,
        audio: true,
      },
    })

    player.value.play(url.value)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  void start()
})

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
  min-width: 360px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 300px));
  gap: 8px;
}
</style>

