<template>
  <div style="width: 300px;height: 300px;">
    <div id="app1"></div>
    <div id="app2"></div>
    <div id="app3"></div>
    <div id="app4"></div>
    <div id="app5"></div>
    <div id="app6"></div>
    <div id="app7"></div>
    <div id="app8"></div>
    <div id="app9"></div>
  </div>
</template>

<script lang="ts" setup>

  setTimeout(()=>{
    // 西瓜播放器资源
    // player.play(document.getElementById(`app${i}`),'http://sf1-cdn-tos.huoshanstatic.com/obj/media-fe/xgplayer_doc_video/flv/xgplayer-demo-720p.flv').catch((err) => {
    //   console.log(err);
    // })

    // go flv-live-server 起的服务
    // play(document.getElementById("app1"),'http://localhost:8080/live.flv')


    play(document.getElementById("app1"),'http://127.0.0.1:8888/live/test/index.m3u8')
  },3000)

  function play(container: HTMLElement | null, url: string){
      const player =  new (window as any).Jessibuca({
          container: container,
          isFlv: true,
          videoBuffer: 0.2, // 缓冲时长
          isResize: false, // 是否自动调整大小
          hasAudio: true, // 是否有音频
          loadingText: '加载中...',
          showBandwidth: false,
          loadingTimeout: 10,
          heartTimeout: 10,
          heartTimeoutReplay: false,
          loadingTimeoutReplay: false,
          supportDblclickFullscreen: true,
          operateBtns: {
            fullscreen: true, // 全屏按钮
            screenshot: true, // 截图按钮
            play: true, // 播放/暂停按钮
            audio: true, // 音频按钮
          },
        });
        
   
      player.play(url).catch((err: unknown) => {
        console.log(err);
      })
  }

</script>
