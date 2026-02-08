<script setup lang="ts">
import type { CacheCategory } from "../types";
import { formatBytes } from "../utils";

defineProps<{
  categories: CacheCategory[];
  selected: string | null;
}>();

const emit = defineEmits<{
  select: [name: string];
}>();

function percentage(size: number, total: number): number {
  if (total === 0) return 0;
  return Math.round((size / total) * 100);
}
</script>

<template>
  <div class="space-y-2">
    <h2 class="text-sm font-medium text-gray-500 uppercase tracking-wide">
      Breakdown
    </h2>
    <div
      v-for="cat in categories"
      :key="cat.name"
      class="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors"
      :class="
        selected === cat.name
          ? 'border-gray-900 bg-gray-50'
          : 'border-gray-200 hover:bg-gray-50'
      "
      @click="emit('select', cat.name)"
    >
      <div class="flex-1 min-w-0">
        <div class="flex items-center justify-between mb-1">
          <span class="text-sm font-medium text-gray-900">{{ cat.name }}</span>
          <span class="text-sm text-gray-500">{{ formatBytes(cat.size) }}</span>
        </div>
        <div class="w-full bg-gray-200 rounded-full h-1.5">
          <div
            class="bg-gray-700 h-1.5 rounded-full transition-all"
            :style="{
              width:
                percentage(
                  cat.size,
                  categories.reduce((sum, c) => sum + c.size, 0),
                ) + '%',
            }"
          />
        </div>
      </div>
      <span class="text-xs text-gray-400 w-16 text-right">
        {{ cat.fileCount.toLocaleString() }} files
      </span>
    </div>
  </div>
</template>
