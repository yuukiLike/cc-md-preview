<script setup lang="ts">
import { useMarkdownStore } from "@/stores/markdown";
import ThemeSelector from "./ThemeSelector.vue";

const emit = defineEmits<{
  openFile: [];
}>();

const store = useMarkdownStore();
</script>

<template>
  <div
    class="flex items-center justify-between px-4 h-12 border-b border-gray-200 bg-white"
  >
    <div class="flex items-center gap-3">
      <button
        class="px-3 py-1.5 text-sm bg-gray-900 text-white rounded-md hover:bg-gray-800 transition-colors"
        @click="emit('openFile')"
      >
        Open File
      </button>
      <span v-if="store.fileName" class="text-sm text-gray-500 truncate max-w-xs">
        {{ store.fileName }}
      </span>
    </div>
    <ThemeSelector />
  </div>
</template>
