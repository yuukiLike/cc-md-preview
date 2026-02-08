import { defineStore } from "pinia";
import { ref } from "vue";
import type { CacheInfo, CacheEntry } from "@/modules/chrome-cache/types";

export const useChromeCacheStore = defineStore("chrome-cache", () => {
  const cacheInfo = ref<CacheInfo | null>(null);
  const selectedCategory = ref<string | null>(null);
  const entries = ref<CacheEntry[]>([]);
  const isScanning = ref(false);
  const isCleaning = ref(false);
  const lastScanTime = ref<Date | null>(null);

  function setCacheInfo(info: CacheInfo) {
    cacheInfo.value = info;
    lastScanTime.value = new Date();
  }

  function setEntries(list: CacheEntry[]) {
    entries.value = list;
  }

  function reset() {
    cacheInfo.value = null;
    entries.value = [];
    selectedCategory.value = null;
  }

  return {
    cacheInfo,
    selectedCategory,
    entries,
    isScanning,
    isCleaning,
    lastScanTime,
    setCacheInfo,
    setEntries,
    reset,
  };
});
