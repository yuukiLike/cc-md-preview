<script setup lang="ts">
import { useMarkdownStore } from "@/stores/markdown";

const emit = defineEmits<{
  openFile: [];
}>();

const store = useMarkdownStore();
</script>

<template>
  <div
    class="relative z-10 flex items-center justify-between px-4 h-12 border-b border-gray-200 bg-white"
  >
    <div class="flex items-center gap-3">
      <button
        class="px-3 py-1.5 text-sm bg-gray-900 text-white rounded-md hover:bg-gray-800 transition-colors"
        @click="emit('openFile')"
      >
        Open File
      </button>
    </div>
    <button
      class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg border transition-all duration-150 cursor-pointer"
      :class="
        store.viewMode === 'theme-selection'
          ? 'bg-blue-100 text-blue-700 border-blue-300'
          : 'border-gray-200 bg-white text-gray-700 hover:shadow-sm hover:-translate-y-px'
      "
      @click="store.toggleViewMode()"
    >
      <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="13.5" cy="6.5" r="2.5" />
        <circle cx="6" cy="12" r="2.5" />
        <circle cx="18" cy="12" r="2.5" />
        <circle cx="8.5" cy="18.5" r="2.5" />
        <circle cx="15.5" cy="18.5" r="2.5" />
      </svg>
      <span class="text-sm font-medium">Themes</span>
    </button>
  </div>
</template>
