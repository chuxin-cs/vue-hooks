<template>
  <div class="page">
    <div class="toolbar">
      <textarea
        v-model="urlsText"
        class="textarea"
        placeholder="粘贴 16 个地址（每行一个，或用空格/逗号分隔）"
        rows="5"
      />
      <div class="actions">
        <button @click="playAll" :disabled="loading">同时播放</button>
        <button @click="stopAll" :disabled="players.length === 0">全部停止</button>
      </div>
    </div>

    <div class="grid grid-4">
      <div v-for="tile in tiles" :key="tile.index" class="cell">
        <div class="cell-title">
          <span>#{{ tile.index }}</span>
          <span class="cell-url">{{ tile.url || "（未设置）" }}</span>
        </div>
        <div :id="tile.playerId" class="player"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from "vue"

const urlsText = ref("")
const loading = ref(false)
const players = ref<any[]>([])

const getJessibuca = () => (window as any)?.Jessibuca

const parsedUrls = computed(() => {
  return urlsText.value
    .split(/[\n\r,，\t ]+/)
    .map((s) => s.trim())
    .filter(Boolean)
    .slice(0, 16)
})

const tiles = computed(() => {
  return Array.from({ length: 16 }).map((_, i) => {
    const index = i + 1
    return {
      index,
      url: parsedUrls.value[i] ?? "",
      playerId: `jessi-grid-16-${index}`,
    }
  })
})

const stopAll = () => {
  for (const p of players.value) {
    p?.destroy?.()
    p?.close?.()
    p?.pause?.()
  }
  players.value = []
}

const playAll = async () => {
  loading.value = true
  try {
    const Jessibuca = getJessibuca()
    if (!Jessibuca) {
      alert("请先加载Jessibuca")
      return
    }

    stopAll()

    for (const tile of tiles.value) {
      if (!tile.url) continue
      const container = document.getElementById(tile.playerId)
      if (!container) continue

      const p = new Jessibuca({
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

      p.play(tile.url)
      players.value.push(p)
    }
  } finally {
    loading.value = false
  }
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
  display: grid;
  gap: 10px;
  margin-bottom: 12px;
}

.textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 8px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 12px;
}

.actions {
  display: flex;
  gap: 10px;
  align-items: center;
  flex-wrap: wrap;
}

.grid {
  display: grid;
  gap: 10px;
}

.grid-4 {
  grid-template-columns: repeat(4, minmax(220px, 1fr));
}

.cell {
  border: 1px solid #e5e5e5;
  border-radius: 10px;
  overflow: hidden;
  background: #fff;
}

.cell-title {
  display: flex;
  gap: 10px;
  align-items: center;
  padding: 8px 10px;
  border-bottom: 1px solid #f0f0f0;
  font-size: 12px;
}

.cell-url {
  color: #666;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.player {
  width: 100%;
  height: 180px;
}
</style>

