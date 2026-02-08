<script setup lang="ts">
import { computed } from "vue";
import { useMarkdownStore } from "@/stores/markdown";

const store = useMarkdownStore();

function tabName(path: string) {
  const parts = path.replace(/\\/g, "/").split("/");
  return parts[parts.length - 1];
}

const tabs = computed(() =>
  store.openFiles.map((f, i) => ({
    index: i,
    name: tabName(f.path),
    path: f.path,
    active: i === store.activeIndex,
  })),
);
</script>

<template>
  <div
    v-if="store.openFiles.length > 0"
    class="flex items-center bg-gray-50 border-b border-gray-200 overflow-x-auto"
  >
    <button
      v-for="tab in tabs"
      :key="tab.path"
      class="group flex items-center gap-1.5 px-3 py-1.5 text-sm border-r border-gray-200 shrink-0 transition-colors duration-100"
      :class="
        tab.active
          ? 'bg-white text-gray-900 border-b-2 border-b-blue-500 -mb-px'
          : 'text-gray-500 hover:bg-gray-100 hover:text-gray-700'
      "
      @click="store.switchTab(tab.index)"
    >
      <span class="truncate max-w-[140px]">{{ tab.name }}</span>
      <span
        class="w-4 h-4 flex items-center justify-center rounded-sm opacity-0 group-hover:opacity-100 hover:bg-gray-300 transition-opacity duration-100 shrink-0"
        @click.stop="store.closeTab(tab.index)"
      >
        <svg class="w-3 h-3" viewBox="0 0 12 12" fill="currentColor">
          <path d="M3.17 3.17a.5.5 0 01.7 0L6 5.3l2.13-2.13a.5.5 0 01.7.7L6.71 6l2.12 2.13a.5.5 0 01-.7.7L6 6.71 3.87 8.83a.5.5 0 01-.7-.7L5.3 6 3.17 3.87a.5.5 0 010-.7z" />
        </svg>
      </span>
    </button>
  </div>
</template>
