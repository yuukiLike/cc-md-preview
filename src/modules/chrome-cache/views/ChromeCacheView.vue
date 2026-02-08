<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useChromeCacheStore } from "@/stores/chrome-cache";
import type { CacheInfo, CacheEntry, CleanResult } from "../types";
import CacheSummaryCard from "../components/CacheSummaryCard.vue";
import CacheBreakdown from "../components/CacheBreakdown.vue";
import CacheEntryList from "../components/CacheEntryList.vue";
import CleanupButton from "../components/CleanupButton.vue";

const store = useChromeCacheStore();

async function scan() {
  store.isScanning = true;
  try {
    const info = await invoke<CacheInfo>("get_cache_info");
    store.setCacheInfo(info);
  } catch (e) {
    console.error("Failed to scan cache:", e);
  } finally {
    store.isScanning = false;
  }
}

async function loadEntries(categoryName: string) {
  store.selectedCategory = categoryName;
  try {
    const list = await invoke<CacheEntry[]>("list_cache_entries", {
      cacheType: categoryName,
    });
    store.setEntries(list);
  } catch (e) {
    console.error("Failed to list entries:", e);
    store.setEntries([]);
  }
}

async function cleanCache(types: string[]) {
  store.isCleaning = true;
  try {
    const result = await invoke<CleanResult>("clean_cache", {
      cacheTypes: types,
    });
    console.log(
      `Cleaned: ${result.deletedFiles} files, freed ${result.freedBytes} bytes`,
    );
    if (result.errors.length > 0) {
      console.warn("Clean errors:", result.errors);
    }
    await scan();
  } catch (e) {
    console.error("Failed to clean cache:", e);
  } finally {
    store.isCleaning = false;
  }
}

onMounted(() => {
  if (!store.cacheInfo) {
    scan();
  }
});
</script>

<template>
  <div class="p-6 max-w-3xl mx-auto space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-lg font-semibold text-gray-900">Chrome Cache</h1>
      <button
        class="px-3 py-1.5 text-sm bg-gray-100 rounded-md hover:bg-gray-200 text-gray-700 transition-colors disabled:opacity-50"
        :disabled="store.isScanning"
        @click="scan"
      >
        {{ store.isScanning ? "Scanning..." : "Refresh" }}
      </button>
    </div>

    <div
      v-if="store.isScanning && !store.cacheInfo"
      class="flex items-center justify-center py-20 text-gray-400"
    >
      <svg
        class="animate-spin w-5 h-5 mr-2"
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
      Scanning cache directories...
    </div>

    <template v-else-if="store.cacheInfo">
      <CacheSummaryCard :info="store.cacheInfo" />
      <CacheBreakdown
        :categories="store.cacheInfo.categories"
        :selected="store.selectedCategory"
        @select="loadEntries"
      />
      <CacheEntryList
        v-if="store.selectedCategory"
        :entries="store.entries"
        :category="store.selectedCategory"
      />
      <CleanupButton
        :categories="store.cacheInfo.categories"
        :is-cleaning="store.isCleaning"
        @clean="cleanCache"
      />
    </template>
  </div>
</template>
