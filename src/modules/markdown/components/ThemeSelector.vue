<script setup lang="ts">
import { useMarkdownStore, type MarkdownTheme } from "@/stores/markdown";

const store = useMarkdownStore();

const themes: { id: MarkdownTheme; label: string }[] = [
  { id: "github", label: "GitHub" },
  { id: "github-dark", label: "GitHub Dark" },
  { id: "minimal", label: "Minimal" },
  { id: "solarized", label: "Solarized" },
];
</script>

<template>
  <select
    :value="store.currentTheme"
    class="text-sm border border-gray-300 rounded-md px-2 py-1 bg-white text-gray-700 focus:outline-none focus:ring-1 focus:ring-gray-400"
    @change="store.setTheme(($event.target as HTMLSelectElement).value as MarkdownTheme)"
  >
    <option v-for="t in themes" :key="t.id" :value="t.id">
      {{ t.label }}
    </option>
  </select>
</template>
