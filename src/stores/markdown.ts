import { defineStore } from "pinia";
import { ref, computed } from "vue";

export type MarkdownTheme =
  | "github"
  | "github-dark"
  | "minimal"
  | "solarized";

export const useMarkdownStore = defineStore("markdown", () => {
  const filePath = ref<string | null>(null);
  const rawContent = ref("");
  const currentTheme = ref<MarkdownTheme>("github");
  const recentFiles = ref<string[]>([]);

  const fileName = computed(() => {
    if (!filePath.value) return null;
    const parts = filePath.value.replace(/\\/g, "/").split("/");
    return parts[parts.length - 1];
  });

  function setFile(path: string, content: string) {
    filePath.value = path;
    rawContent.value = content;
    recentFiles.value = [
      path,
      ...recentFiles.value.filter((f) => f !== path),
    ].slice(0, 10);
  }

  function setTheme(theme: MarkdownTheme) {
    currentTheme.value = theme;
  }

  function clear() {
    filePath.value = null;
    rawContent.value = "";
  }

  return {
    filePath,
    rawContent,
    currentTheme,
    recentFiles,
    fileName,
    setFile,
    setTheme,
    clear,
  };
});
