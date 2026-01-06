import { createRouter, createWebHashHistory } from "vue-router"

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/hls",
    },
    {
      path: "/hls",
      name: "hls",
      component: () => import("../pages/hls/HlsPage.vue"),
    },
    {
      path: "/flv",
      name: "flv",
      component: () => import("../pages/flv/FlvPage.vue"),
    },
    {
      path: "/jessibuca",
      name: "jessibuca",
      component: () => import("../pages/jessibuca/JessibucaPage.vue"),
    },
  ],
})

export default router
