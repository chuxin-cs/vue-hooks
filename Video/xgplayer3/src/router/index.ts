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
      path: "/flv-grid",
      name: "flv-grid",
      component: () => import("../pages/flv/FlvGridPage.vue"),
    },
    {
      path: "/jessibuca",
      name: "jessibuca",
      component: () => import("../pages/jessibuca/JessibucaPage.vue"),
    },
    {
      path: "/jessibuca-grid-9",
      name: "jessibuca-grid-9",
      component: () => import("../pages/jessibuca/JessibucaGrid9Page.vue"),
    },
    {
      path: "/jessibuca-grid-16",
      name: "jessibuca-grid-16",
      component: () => import("../pages/jessibuca/JessibucaGrid16Page.vue"),
    },
  ],
})

export default router
