<script setup lang="ts">
import { ref, toRef } from "vue";
import { useMarkdownStore } from "@/stores/markdown";
import { useMarkdownRenderer } from "../composables/useMarkdownRenderer";
import { useMermaid } from "../composables/useMermaid";

const store = useMarkdownStore();
const { rendered } = useMarkdownRenderer(toRef(store, "rawContent"));
const previewRef = ref<HTMLElement | null>(null);

useMermaid(previewRef, rendered, toRef(store, "currentTheme"));
</script>

<template>
  <div
    ref="previewRef"
    class="markdown-body h-full overflow-auto p-8"
    :class="`theme-${store.currentTheme}`"
    v-html="rendered"
  />
</template>

<style scoped>
.markdown-body {
  transition: background-color 0.3s ease, color 0.3s ease;
}
.markdown-body :deep(pre),
.markdown-body :deep(code),
.markdown-body :deep(th),
.markdown-body :deep(td),
.markdown-body :deep(tr),
.markdown-body :deep(blockquote),
.markdown-body :deep(hr),
.markdown-body :deep(h1),
.markdown-body :deep(h2),
.markdown-body :deep(h3),
.markdown-body :deep(h4),
.markdown-body :deep(h5),
.markdown-body :deep(h6),
.markdown-body :deep(a) {
  transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease;
}
.markdown-body :deep(.mermaid-block) {
  margin: 16px 0;
  text-align: center;
}
.markdown-body :deep(.mermaid-block svg) {
  max-width: 100%;
  height: auto;
}
</style>
