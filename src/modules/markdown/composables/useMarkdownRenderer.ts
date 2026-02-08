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

export function useMarkdownRenderer(source: Ref<string>) {
  const rendered = computed(() => {
    if (!source.value) return "";
    return md.render(source.value);
  });

  return { rendered };
}
