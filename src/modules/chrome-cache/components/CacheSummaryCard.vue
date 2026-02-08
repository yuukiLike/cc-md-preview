<script setup lang="ts">
import type { CacheInfo } from "../types";
import { formatBytes } from "../utils";

defineProps<{
  info: CacheInfo;
}>();
</script>

<template>
  <div class="bg-gray-50 rounded-xl p-5 border border-gray-200">
    <div class="flex items-baseline gap-3">
      <span class="text-3xl font-bold text-gray-900">{{
        formatBytes(info.totalSize)
      }}</span>
      <span class="text-sm text-gray-500"
        >{{ info.totalFiles.toLocaleString() }} files</span
      >
    </div>
    <p class="text-sm text-gray-500 mt-1">
      Profile: {{ info.profile }}
    </p>
  </div>
</template>
