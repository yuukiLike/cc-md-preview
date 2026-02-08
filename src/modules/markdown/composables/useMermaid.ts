import { watch, nextTick, type Ref } from "vue";
import type { MarkdownTheme } from "@/stores/markdown";

const DARK_THEMES: MarkdownTheme[] = [
  "github-dark",
  "dracula",
  "nord",
  "cobalt",
  "monokai",
  "terminal",
  "sunset",
];

let mermaidModule: typeof import("mermaid") | null = null;

async function ensureMermaid() {
  if (!mermaidModule) {
    mermaidModule = await import("mermaid");
  }
  return mermaidModule.default;
}

export function useMermaid(
  containerRef: Ref<HTMLElement | null>,
  renderedHtml: Ref<string>,
  currentTheme: Ref<MarkdownTheme>,
) {
  let renderCount = 0;

  async function renderMermaidBlocks() {
    const container = containerRef.value;
    if (!container) return;

    const blocks = container.querySelectorAll<HTMLElement>(".mermaid-block");
    if (blocks.length === 0) return;

    const mermaid = await ensureMermaid();
    const isDark = DARK_THEMES.includes(currentTheme.value);

    mermaid.initialize({
      startOnLoad: false,
      theme: isDark ? "dark" : "default",
      securityLevel: "strict",
    });

    for (const block of blocks) {
      const source = block.dataset.source;
      if (!source) continue;

      const decoded = source
        .replace(/&quot;/g, '"')
        .replace(/&gt;/g, ">")
        .replace(/&lt;/g, "<")
        .replace(/&amp;/g, "&");

      const id = `mermaid-${++renderCount}`;
      try {
        const { svg } = await mermaid.render(id, decoded);
        block.innerHTML = svg;
      } catch {
        block.innerHTML = `<pre class="mermaid-error" style="color:#ef4444;font-size:13px;">Failed to render mermaid diagram</pre>`;
      }
    }
  }

  watch(
    [renderedHtml, currentTheme],
    () => {
      nextTick(() => {
        // Restore source text on theme change before re-render
        const container = containerRef.value;
        if (container) {
          const blocks = container.querySelectorAll<HTMLElement>(".mermaid-block");
          for (const block of blocks) {
            if (block.dataset.source) {
              block.innerHTML = "";
            }
          }
        }
        renderMermaidBlocks();
      });
    },
    { flush: "post" },
  );
}
