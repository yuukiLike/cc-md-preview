<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useMarkdownStore, type MarkdownFileInfo } from "@/stores/markdown";
import { useFileDrop } from "../composables/useFileDrop";
import { useTabKeyCycling } from "../composables/useTabKeyCycling";
import MarkdownToolbar from "../components/MarkdownToolbar.vue";
import MarkdownPreview from "../components/MarkdownPreview.vue";
import TabBar from "../components/TabBar.vue";
import ThemeGrid from "../components/ThemeGrid.vue";
import FolderCardGrid from "../components/FolderCardGrid.vue";
import DropZone from "../components/DropZone.vue";

const store = useMarkdownStore();

useTabKeyCycling();

const MD_EXTENSIONS = [".md", ".markdown", ".mdx"];

function isMarkdownFile(path: string): boolean {
  const lower = path.toLowerCase();
  return MD_EXTENSIONS.some((ext) => lower.endsWith(ext));
}

async function loadFile(path: string) {
  try {
    const content = await invoke<string>("read_markdown_file", { path });
    store.setFile(path, content);
    store.setViewMode("preview");
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

async function scanFolder(folderPath: string) {
  store.isScanningFolder = true;
  try {
    const files = await invoke<MarkdownFileInfo[]>("list_markdown_files", {
      folderPath,
    });
    store.setFolderFiles(folderPath, files);
  } catch (e) {
    console.error("Failed to scan folder:", e);
  } finally {
    store.isScanningFolder = false;
  }
}

async function openFolder() {
  try {
    const path = await invoke<string | null>("open_folder_dialog");
    if (path) {
      await scanFolder(path);
    }
  } catch (e) {
    console.error("Failed to open folder dialog:", e);
  }
}

async function openFileFromFolder(path: string) {
  await loadFile(path);
}

const { isDragging } = useFileDrop(async (paths) => {
  // Separate markdown files and potential folders
  const mdFiles = paths.filter(isMarkdownFile);
  const otherPaths = paths.filter((p) => !isMarkdownFile(p));

  // Load individual markdown files directly
  for (const path of mdFiles) {
    await loadFile(path);
  }

  // Try scanning non-md paths as folders (first one wins)
  if (otherPaths.length > 0 && mdFiles.length === 0) {
    await scanFolder(otherPaths[0]);
  }
});
</script>

<template>
  <div class="relative flex flex-col h-full">
    <MarkdownToolbar @open-file="openFile" @open-folder="openFolder" />
    <TabBar />
    <div class="flex-1 overflow-hidden relative">
      <Transition name="view-fade" mode="out-in">
        <!-- Scanning loading -->
        <div
          v-if="store.isScanningFolder"
          key="scanning"
          class="flex flex-col items-center justify-center h-full text-gray-400 gap-3"
        >
          <svg
            class="w-8 h-8 animate-spin"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
            />
          </svg>
          <p class="text-sm">Scanning folder…</p>
        </div>

        <!-- Theme selection -->
        <ThemeGrid v-else-if="store.viewMode === 'theme-selection'" key="themes" />

        <!-- Folder card grid -->
        <FolderCardGrid
          v-else-if="store.viewMode === 'folder'"
          key="folder"
          @open-file="openFileFromFolder"
        />

        <!-- Markdown preview -->
        <MarkdownPreview v-else-if="store.rawContent" key="preview" />

        <!-- Empty state -->
        <div
          v-else
          key="empty"
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
          <p class="text-sm">Open a file or folder, or drag it here</p>
          <div class="flex gap-2 mt-2">
            <button
              class="px-4 py-2 text-sm bg-gray-100 rounded-lg hover:bg-gray-200 text-gray-600 transition-colors"
              @click="openFile"
            >
              Open File
            </button>
            <button
              class="px-4 py-2 text-sm bg-gray-100 rounded-lg hover:bg-gray-200 text-gray-600 transition-colors"
              @click="openFolder"
            >
              Open Folder
            </button>
          </div>
        </div>
      </Transition>
      <DropZone :active="isDragging" />
    </div>
  </div>
</template>

<style scoped>
.view-fade-enter-active,
.view-fade-leave-active {
  transition: opacity 0.15s ease;
}
.view-fade-enter-from,
.view-fade-leave-to {
  opacity: 0;
}
</style>
