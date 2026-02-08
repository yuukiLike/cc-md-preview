<script setup lang="ts">
import { computed } from "vue";
import type { CacheEntry } from "../types";
import { formatBytes } from "../utils";

const props = defineProps<{
  entries: CacheEntry[];
  category: string;
}>();

const displayEntries = computed(() => props.entries.slice(0, 100));
</script>

<template>
  <div class="space-y-2">
    <div class="flex items-center justify-between">
      <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wide">
        {{ category }} entries
      </h2>
      <span class="text-xs text-gray-400">
        Showing {{ displayEntries.length }} of
        {{ entries.length.toLocaleString() }}
      </span>
    </div>
    <div
      class="border border-gray-200 rounded-lg overflow-hidden max-h-64 overflow-y-auto"
    >
      <div
        v-if="entries.length === 0"
        class="p-4 text-sm text-gray-400 text-center"
      >
        No entries found
      </div>
      <div
        v-for="entry in displayEntries"
        :key="entry.name"
        class="flex items-center justify-between px-3 py-2 border-b border-gray-100 last:border-0 text-sm"
      >
        <span class="text-gray-700 truncate max-w-md" :title="entry.name">{{
          entry.name
        }}</span>
        <span class="text-gray-400 shrink-0 ml-3">{{
          formatBytes(entry.size)
        }}</span>
      </div>
    </div>
  </div>
</template>
