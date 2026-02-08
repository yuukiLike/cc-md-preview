import MarkdownIt from "markdown-it";
import hljs from "highlight.js";
import { computed, type Ref } from "vue";

const md = new MarkdownIt({
  html: false,
  linkify: true,
  typographer: true,
  highlight(str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value;
      } catch {
        // fallback
      }
    }
    return "";
  },
});

// Override fence rule to handle mermaid code blocks
const defaultFence = md.renderer.rules.fence!;
md.renderer.rules.fence = (tokens, idx, options, env, self) => {
  const token = tokens[idx];
  const info = token.info.trim();
  if (info === "mermaid") {
    const encoded = token.content
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
    return `<div class="mermaid-block" data-source="${encoded}"></div>`;
  }
  return defaultFence(tokens, idx, options, env, self);
};

export function useMarkdownRenderer(source: Ref<string>) {
  const rendered = computed(() => {
    if (!source.value) return "";
    return md.render(source.value);
  });

  return { rendered };
}
