import { getCurrentWebview } from "@tauri-apps/api/webview";
import { onMounted, onUnmounted, ref } from "vue";

export function useFileDrop(onDrop: (paths: string[]) => void) {
  const isDragging = ref(false);
  let unlisten: (() => void) | null = null;

  onMounted(async () => {
    unlisten = await getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === "over") {
        isDragging.value = true;
      } else if (event.payload.type === "drop") {
        isDragging.value = false;
        const paths = event.payload.paths ?? [];
        const mdFiles = paths.filter(
          (p: string) =>
            p.endsWith(".md") ||
            p.endsWith(".markdown") ||
            p.endsWith(".mdx"),
        );
        if (mdFiles.length > 0) {
          onDrop(mdFiles);
        }
      } else {
        isDragging.value = false;
      }
    });
  });

  onUnmounted(() => {
    unlisten?.();
  });

  return { isDragging };
}
