import { defineStore } from "pinia";
import { ref, computed } from "vue";

export type MarkdownTheme =
  | "github"
  | "github-dark"
  | "solarized"
  | "dracula"
  | "nord"
  | "cobalt"
  | "monokai"
  | "terminal"
  | "sunset"
  | "arctic";

export interface TabFile {
  path: string;
  content: string;
}

export type ViewMode = "preview" | "theme-selection";

export const useMarkdownStore = defineStore("markdown", () => {
  const openFiles = ref<TabFile[]>([]);
  const activeIndex = ref(0);
  const currentTheme = ref<MarkdownTheme>("github");
  const recentFiles = ref<string[]>([]);
  const viewMode = ref<ViewMode>("preview");

  const filePath = computed(() => openFiles.value[activeIndex.value]?.path ?? null);
  const rawContent = computed(() => openFiles.value[activeIndex.value]?.content ?? "");
  const fileName = computed(() => {
    if (!filePath.value) return null;
    const parts = filePath.value.replace(/\\/g, "/").split("/");
    return parts[parts.length - 1];
  });

  function addFile(path: string, content: string) {
    const existingIdx = openFiles.value.findIndex((f) => f.path === path);
    if (existingIdx >= 0) {
      openFiles.value[existingIdx].content = content;
      activeIndex.value = existingIdx;
    } else {
      openFiles.value.push({ path, content });
      activeIndex.value = openFiles.value.length - 1;
    }
    recentFiles.value = [
      path,
      ...recentFiles.value.filter((f) => f !== path),
    ].slice(0, 10);
  }

  function switchTab(index: number) {
    if (index >= 0 && index < openFiles.value.length) {
      activeIndex.value = index;
    }
  }

  function closeTab(index: number) {
    if (index < 0 || index >= openFiles.value.length) return;
    openFiles.value.splice(index, 1);
    if (openFiles.value.length === 0) {
      activeIndex.value = 0;
    } else if (activeIndex.value >= openFiles.value.length) {
      activeIndex.value = openFiles.value.length - 1;
    } else if (activeIndex.value > index) {
      activeIndex.value--;
    }
  }

  function nextTab() {
    if (openFiles.value.length <= 1) return;
    activeIndex.value = (activeIndex.value + 1) % openFiles.value.length;
  }

  function prevTab() {
    if (openFiles.value.length <= 1) return;
    activeIndex.value =
      (activeIndex.value - 1 + openFiles.value.length) % openFiles.value.length;
  }

  /** Backward-compatible alias */
  function setFile(path: string, content: string) {
    addFile(path, content);
  }

  function setTheme(theme: MarkdownTheme) {
    currentTheme.value = theme;
  }

  function clear() {
    openFiles.value = [];
    activeIndex.value = 0;
  }

  function toggleViewMode() {
    viewMode.value = viewMode.value === "preview" ? "theme-selection" : "preview";
  }

  function setViewMode(mode: ViewMode) {
    viewMode.value = mode;
  }

  return {
    openFiles,
    activeIndex,
    filePath,
    rawContent,
    currentTheme,
    recentFiles,
    viewMode,
    fileName,
    addFile,
    switchTab,
    closeTab,
    nextTab,
    prevTab,
    setFile,
    setTheme,
    clear,
    toggleViewMode,
    setViewMode,
  };
});
