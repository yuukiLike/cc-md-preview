<script setup lang="ts">
import { useMarkdownStore } from "@/stores/markdown";
import { useMarkdownRenderer } from "../composables/useMarkdownRenderer";
import { toRef } from "vue";

const store = useMarkdownStore();
const { rendered } = useMarkdownRenderer(toRef(store, "rawContent"));
</script>

<template>
  <div
    class="markdown-body h-full overflow-auto p-8"
    :class="`theme-${store.currentTheme}`"
    v-html="rendered"
  />
</template>
