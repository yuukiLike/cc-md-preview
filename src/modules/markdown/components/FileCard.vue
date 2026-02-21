<script setup lang="ts">
import { computed } from "vue";
import type { MarkdownFileInfo } from "@/stores/markdown";

const props = defineProps<{
  file: MarkdownFileInfo;
  index: number;
}>();

defineEmits<{
  open: [path: string];
}>();

const animationDelay = computed(() => `${Math.min(props.index * 50, 500)}ms`);

const isSubdir = computed(() => props.file.relativePath.includes("/"));

const displaySize = computed(() => {
  const bytes = props.file.size;
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
});

const displayDate = computed(() => {
  try {
    const d = new Date(props.file.modified);
    return d.toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" });
  } catch {
    return "";
  }
});

/** Strip markdown heading markers and emphasis for cleaner preview */
const cleanPreview = computed(() =>
  props.file.preview
    .replace(/^#{1,6}\s+/gm, "")
    .replace(/\*\*(.+?)\*\*/g, "$1")
    .replace(/\*(.+?)\*/g, "$1")
    .replace(/`(.+?)`/g, "$1")
    .replace(/\[(.+?)\]\(.+?\)/g, "$1"),
);
</script>

<template>
  <button
    class="file-card group w-full text-left rounded-xl border border-gray-200 bg-white p-4 transition-all duration-150 cursor-pointer hover:shadow-md hover:-translate-y-0.5 active:scale-[0.98]"
    :style="{ animationDelay }"
    @click="$emit('open', file.path)"
  >
    <!-- File name -->
    <h3 class="text-sm font-semibold text-gray-900 truncate mb-0.5">
      {{ file.name }}
    </h3>

    <!-- Relative path (if in subdirectory) -->
    <p v-if="isSubdir" class="text-xs text-gray-400 truncate mb-2">
      {{ file.relativePath }}
    </p>
    <div v-else class="mb-2" />

    <!-- Preview text -->
    <p class="text-xs text-gray-500 leading-relaxed line-clamp-3 min-h-[3.75rem]">
      {{ cleanPreview || "Empty file" }}
    </p>

    <!-- Footer meta -->
    <div class="mt-3 flex items-center gap-3 text-[11px] text-gray-400">
      <span>{{ file.wordCount.toLocaleString() }} words</span>
      <span class="w-px h-3 bg-gray-200" />
      <span>{{ displaySize }}</span>
      <span class="w-px h-3 bg-gray-200" />
      <span>{{ displayDate }}</span>
    </div>
  </button>
</template>

<style scoped>
.file-card {
  animation: fade-up 0.3s ease both;
}

@keyframes fade-up {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
