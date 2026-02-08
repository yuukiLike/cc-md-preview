<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useMarkdownStore } from "@/stores/markdown";
import { useFileDrop } from "../composables/useFileDrop";
import { useTabKeyCycling } from "../composables/useTabKeyCycling";
import MarkdownToolbar from "../components/MarkdownToolbar.vue";
import MarkdownPreview from "../components/MarkdownPreview.vue";
import TabBar from "../components/TabBar.vue";
import ThemeGrid from "../components/ThemeGrid.vue";
import DropZone from "../components/DropZone.vue";

const store = useMarkdownStore();

useTabKeyCycling();

async function loadFile(path: string) {
  try {
    const content = await invoke<string>("read_markdown_file", { path });
    store.setFile(path, content);
  } catch (e) {
    console.error("Failed to load file:", e);
  }
}

async function openFile() {
  try {
    const path = await invoke<string | null>("open_markdown_dialog");
    if (path) {
      await loadFile(path);
    }
  } catch (e) {
    console.error("Failed to open dialog:", e);
  }
}

const { isDragging } = useFileDrop(async (paths) => {
  for (const path of paths) {
    await loadFile(path);
  }
});
</script>

<template>
  <div class="relative flex flex-col h-full">
    <MarkdownToolbar @open-file="openFile" />
    <TabBar />
    <div class="flex-1 overflow-hidden relative">
      <template v-if="store.viewMode === 'theme-selection'">
        <ThemeGrid />
      </template>
      <template v-else-if="store.rawContent">
        <MarkdownPreview />
      </template>
      <template v-else>
        <div
          class="flex flex-col items-center justify-center h-full text-gray-400 gap-3"
        >
          <svg
            class="w-12 h-12"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="1.5"
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
          <p class="text-sm">Open a .md file or drag it here</p>
          <button
            class="mt-2 px-4 py-2 text-sm bg-gray-100 rounded-lg hover:bg-gray-200 text-gray-600 transition-colors"
            @click="openFile"
          >
            Open File
          </button>
        </div>
      </template>
      <DropZone :active="isDragging" />
    </div>
  </div>
</template>
