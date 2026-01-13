<template>
  <div class="app">
    <header class="header">
      <nav class="nav">
        <router-link to="/hls" class="link">HLS</router-link>
        <router-link to="/flv" class="link">FLV</router-link>
        <router-link to="/flv-grid" class="link">FLV 9宫格</router-link>
        <router-link to="/jessibuca" class="link">Jessibuca</router-link>
      </nav>
    </header>
    <main class="main">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts"></script>


<style>
html,
#app,
body {
  height: 100vh;
  width: 100vw;
}

.app {
  width: 100%;
  height: 100%;
}

.header {
  padding: 12px;
  border-bottom: 1px solid #e5e5e5;
}

.nav {
  display: flex;
  gap: 12px;
}

.link {
  color: #333;
  text-decoration: none;
  padding: 6px 10px;
  border-radius: 8px;
  border: 1px solid #e5e5e5;
}

.link.router-link-active {
  background: #f3f3f3;
}

.main {
  height: calc(100% - 56px);
  overflow: auto;
}
</style>
