<script setup lang="ts">
import { ref, computed } from "vue";
import { useMarkdownStore } from "@/stores/markdown";
import FileCard from "./FileCard.vue";

const emit = defineEmits<{
  openFile: [path: string];
}>();

const store = useMarkdownStore();
const search = ref("");

const folderName = computed(() => {
  if (!store.folderPath) return "";
  const parts = store.folderPath.replace(/\\/g, "/").split("/");
  return parts[parts.length - 1] || parts[parts.length - 2] || "";
});

const filteredFiles = computed(() => {
  const q = search.value.toLowerCase().trim();
  if (!q) return store.folderFiles;
  return store.folderFiles.filter(
    (f) =>
      f.name.toLowerCase().includes(q) ||
      f.relativePath.toLowerCase().includes(q) ||
      f.preview.toLowerCase().includes(q),
  );
});
</script>

<template>
  <div class="h-full overflow-y-auto p-6 bg-gray-50">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center gap-3 mb-5">
      <div class="flex-1 min-w-0">
        <h2 class="text-base font-semibold text-gray-900 truncate">
          {{ folderName }}
        </h2>
        <p class="text-xs text-gray-400 mt-0.5">
          {{ store.folderFiles.length }} file{{ store.folderFiles.length !== 1 ? "s" : "" }}
        </p>
      </div>
      <div class="relative">
        <svg
          class="absolute left-2.5 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400 pointer-events-none"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
          />
        </svg>
        <input
          v-model="search"
          type="text"
          placeholder="Filter files…"
          class="w-full sm:w-56 pl-8 pr-3 py-1.5 text-sm border border-gray-200 rounded-lg bg-white focus:outline-none focus:border-gray-400 transition-colors"
        />
      </div>
    </div>

    <!-- Card grid -->
    <div
      v-if="filteredFiles.length > 0"
      class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4"
    >
      <FileCard
        v-for="(file, i) in filteredFiles"
        :key="file.path"
        :file="file"
        :index="i"
        @open="emit('openFile', $event)"
      />
    </div>

    <!-- Empty state -->
    <div
      v-else
      class="flex flex-col items-center justify-center py-20 text-gray-400"
    >
      <svg class="w-10 h-10 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="1.5"
          d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <p class="text-sm">
        {{ search ? "No files match your search" : "No Markdown files found in this folder" }}
      </p>
    </div>
  </div>
</template>
