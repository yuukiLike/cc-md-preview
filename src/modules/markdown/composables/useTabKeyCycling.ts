import { onMounted, onUnmounted } from "vue";
import { useMarkdownStore } from "@/stores/markdown";

const SKIP_TAGS = new Set(["INPUT", "TEXTAREA", "SELECT"]);

export function useTabKeyCycling() {
  const store = useMarkdownStore();

  function onKeyDown(e: KeyboardEvent) {
    if (e.key !== "Tab") return;
    if (e.ctrlKey || e.metaKey || e.altKey) return;
    if (store.openFiles.length <= 1) return;

    const target = e.target as HTMLElement;
    if (SKIP_TAGS.has(target.tagName) || target.isContentEditable) return;

    e.preventDefault();
    if (e.shiftKey) {
      store.prevTab();
    } else {
      store.nextTab();
    }
  }

  onMounted(() => window.addEventListener("keydown", onKeyDown));
  onUnmounted(() => window.removeEventListener("keydown", onKeyDown));
}
