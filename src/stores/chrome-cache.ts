import { defineStore } from "pinia";
import { ref } from "vue";
import type {
  CacheInfo,
  CacheEntry,
  BrowserInfo,
} from "@/modules/chrome-cache/types";

export const useChromeCacheStore = defineStore("chrome-cache", () => {
  const cacheInfo = ref<CacheInfo | null>(null);
  const selectedCategory = ref<string | null>(null);
  const entries = ref<CacheEntry[]>([]);
  const isScanning = ref(false);
  const isCleaning = ref(false);
  const lastScanTime = ref<Date | null>(null);
  const browsers = ref<BrowserInfo[]>([]);
  const activeBrowser = ref("Chrome");

  function setCacheInfo(info: CacheInfo) {
    cacheInfo.value = info;
    lastScanTime.value = new Date();
  }

  function setEntries(list: CacheEntry[]) {
    entries.value = list;
  }

  function setBrowsers(list: BrowserInfo[]) {
    browsers.value = list;
  }

  function setActiveBrowser(name: string) {
    activeBrowser.value = name;
    cacheInfo.value = null;
    entries.value = [];
    selectedCategory.value = null;
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
    browsers,
    activeBrowser,
    setCacheInfo,
    setEntries,
    setBrowsers,
    setActiveBrowser,
    reset,
  };
});
