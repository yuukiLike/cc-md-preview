<script setup lang="ts">
import { useRoute } from "vue-router";

const route = useRoute();

const modules = [
  {
    id: "markdown",
    name: "Markdown",
    path: "/markdown",
    icon: "M",
  },
  {
    id: "chrome-cache",
    name: "Cache",
    path: "/chrome-cache",
    icon: "C",
  },
];
</script>

<template>
  <aside
    class="w-14 flex flex-col items-center py-4 gap-2 border-r border-gray-200 bg-gray-50/80"
  >
    <RouterLink
      v-for="mod in modules"
      :key="mod.id"
      :to="mod.path"
      :title="mod.name"
      class="w-10 h-10 flex items-center justify-center rounded-lg text-sm font-medium transition-colors"
      :class="
        route.path.startsWith(mod.path)
          ? 'bg-gray-900 text-white'
          : 'text-gray-500 hover:bg-gray-200 hover:text-gray-900'
      "
    >
      {{ mod.icon }}
    </RouterLink>
  </aside>
</template>
