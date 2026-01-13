<template>
  <div class="page">
    <div class="toolbar">
      <textarea
        v-model="urlsText"
        class="textarea"
        placeholder="粘贴 9 个 FLV 地址（每行一个，或用空格/逗号分隔）"
        rows="4"
      />

      <div class="actions">
        <button @click="playAll" :disabled="loading">同时播放</button>
        <button @click="stopAll" :disabled="players.length === 0">全部停止</button>
        <label class="toggle">
          <input v-model="muted" type="checkbox" />
          静音（建议开启，避免浏览器阻止自动播放）
        </label>
      </div>
    </div>

    <div class="grid">
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
import Player from "xgplayer"
import "xgplayer/dist/index.min.css"
import FlvPlugin from "xgplayer-flv"

const urlsText = ref("")
const loading = ref(false)
const muted = ref(true)

const players = ref<Player[]>([])

const parsedUrls = computed(() => {
  return urlsText.value
    .split(/[\n\r,，\t ]+/)
    .map((s) => s.trim())
    .filter(Boolean)
    .slice(0, 9)
})

const tiles = computed(() => {
  return Array.from({ length: 9 }).map((_, i) => {
    const index = i + 1
    return {
      index,
      url: parsedUrls.value[i] ?? "",
      playerId: `flv-grid-${index}`,
    }
  })
})

const stopAll = () => {
  for (const p of players.value) {
    p.destroy()
  }
  players.value = []
}

const playAll = async () => {
  loading.value = true
  try {
    stopAll()

    for (const tile of tiles.value) {
      if (!tile.url) continue

      const p = new Player({
        id: tile.playerId,
        url: tile.url,
        isLive: true,
        autoplay: true,
        muted: muted.value,
        height: "100%",
        width: "100%",
        plugins: [FlvPlugin],
      })
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

.toggle {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  user-select: none;
}

.grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(240px, 1fr));
  gap: 10px;
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
  aspect-ratio: 16 / 9;
  width: 100%;
}
</style>

