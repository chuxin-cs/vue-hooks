<template>
  <div class="app-container">
    <div id="mse1"></div>
    <div id="mse2"></div>
    <div id="mse3"></div>
    <div id="mse4"></div>
    <div id="mse5"></div>
    <div id="mse6"></div>
    <div id="mse7"></div>
    <div id="mse8"></div>
    <div id="mse9"></div>
    <div id="mse10"></div>
    <div id="mse11"></div>
    <div id="mse12"></div>
    <div id="mse13"></div>
    <div id="mse14"></div>
    <div id="mse15"></div>
    <div id="mse16"></div>
    <div id="mse17"></div>
    <div id="mse18"></div>
    <div id="mse19"></div>
    <div id="mse20"></div>
    <div id="mse21"></div>
    <!-- <APlayer/> -->


    <button @click="playHls()">HLS播放</button>
    <button @click="playFlv()">FLV播放</button>
  </div>
</template>

<script setup lang="ts">
// import APlayer from "./a.vue"
import axios from "axios"
import Player from 'xgplayer';
import 'xgplayer/dist/index.min.css';

import FlvPlugin from 'xgplayer-flv'
import HlsPlugin from 'xgplayer-hls'


// flv的格式在10多个宫格中会出现卡顿 我现在测试一下 hls

// ==================== flv
function playFlv(){
  // 
  axios
    .get("http://127.0.0.1:8080/index/api/addStreamProxy", {
      params: {
        url: "",
        stream: "test",
        app: "live",
        vhost: "__defaultVhost__",
        secret: "123456",

        enable_hls: 0,
        enable_mp4: 0,
        enable_rtsp: 0,
        enable_rtmp: 0
      },
    })
    .then(({ data }) => {
      console.log(data)
      plays("mse1")
    })
}

function plays(id: string){
 new Player({
    id: id,
    
    // 西瓜播放器的视频
    // url: 'http://sf1-cdn-tos.huoshanstatic.com/obj/media-fe/xgplayer_doc_video/flv/xgplayer-demo-720p.flv',
    
    // 本地
    // url: '../public/flv/202512241446_aac.flv',

    // go flv-live-server 起的服务
    // url:'http://localhost:8080/live.flv',
    // isLive: true,

    // url:'',

    height: '200px',
    width: '200px',
    plugins: [FlvPlugin],
  });
}


// ==================== hls
function playHls(){
 play("mse1")
  play("mse2")
  play("mse3")
  play("mse4")
  play("mse5")
  play("mse6")
  play("mse7")
  play("mse8")
  play("mse9")
  play("mse10")
  play("mse11")
  play("mse12")
  play("mse13")
  play("mse14")
  play("mse15")
  play("mse16")
  play("mse17")
  play("mse18")
  play("mse19")
  play("mse20")
  play("mse21")
}
function play(id: string){
  new Player({
    id,
    url:'http://127.0.0.1:8888/live/test/index.m3u8',
    height: '200px',
    width: '200px',
    plugins: [FlvPlugin,HlsPlugin],
    // isLive: true,
    autoplay: true,
    muted: true,

    // ⭐⭐⭐ 核心参数 ⭐⭐⭐
    hls: {
      // ====== Live 行为 ======
  liveSyncDuration: 1,           // 距离 live 1s
  liveMaxLatencyDuration: 2,     // 超过 2s 强制追帧

  // ====== Buffer 硬限制 ======
  maxBufferLength: 2,            // ⭐ 最大前向 buffer（秒）
  maxMaxBufferLength: 4,         // ⭐ 上限兜底
  backBufferLength: 0,           // ⭐ 不保留历史

  // ====== LL-HLS ======
  lowLatencyMode: true,
  enableWorker: true,

  // ====== 网络容错 ======
  fragLoadingTimeOut: 20000,
  manifestLoadingTimeOut: 10000,
  fragLoadingRetryDelay: 1000,
  manifestLoadingRetryDelay: 1000,

  // ====== 彻底防内存增长 ======
  appendErrorMaxRetry: 3,
    },

    preloadTime: 1,
  });
}




</script>


<style>
html,
#app,
body {
  height: 100vh;
  height: 100vw;
}

.app-container {
  width: 100%;
  height: 100%;

}

.app-container>div {
  display: inline-block;
}
</style>
