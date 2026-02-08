import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/markdown",
    },
    {
      path: "/markdown",
      name: "markdown",
      component: () => import("@/modules/markdown/views/MarkdownView.vue"),
      meta: { title: "Markdown Preview", icon: "markdown" },
    },
    {
      path: "/chrome-cache",
      name: "chrome-cache",
      component: () =>
        import("@/modules/chrome-cache/views/ChromeCacheView.vue"),
      meta: { title: "Chrome Cache", icon: "cache" },
    },
  ],
});

export default router;
