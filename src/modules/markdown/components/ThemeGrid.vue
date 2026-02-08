<script setup lang="ts">
import { useMarkdownStore, type MarkdownTheme } from "@/stores/markdown";
import { LIGHT_THEMES, DARK_THEMES } from "../constants/themes";

const store = useMarkdownStore();

function select(id: MarkdownTheme) {
  store.setTheme(id);
  store.setViewMode("preview");
}
</script>

<template>
  <div class="h-full overflow-y-auto p-6 bg-gray-50">
    <!-- Light themes -->
    <div class="mb-6">
      <h3 class="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-3">
        Light
      </h3>
      <div class="grid grid-cols-3 gap-3">
        <button
          v-for="t in LIGHT_THEMES"
          :key="t.id"
          class="rounded-xl border-2 p-3 transition-all duration-150 cursor-pointer hover:shadow-md hover:-translate-y-0.5"
          :class="
            store.currentTheme === t.id
              ? 'border-blue-500 shadow-md'
              : 'border-gray-200 hover:border-gray-300'
          "
          @click="select(t.id)"
        >
          <!-- Color stripes -->
          <div class="flex h-4 rounded-md overflow-hidden mb-2.5 shadow-sm">
            <span class="flex-1" :style="{ background: t.colors.bg }" />
            <span class="flex-1" :style="{ background: t.colors.text }" />
            <span class="flex-1" :style="{ background: t.colors.accent }" />
          </div>
          <!-- Mini preview -->
          <div
            class="rounded-md p-2 mb-2"
            :style="{ background: t.colors.bg }"
          >
            <div
              class="h-2 w-3/4 rounded-sm mb-1.5"
              :style="{ background: t.colors.text, opacity: 0.8 }"
            />
            <div
              class="h-1.5 w-full rounded-sm mb-1"
              :style="{ background: t.colors.text, opacity: 0.2 }"
            />
            <div
              class="h-1.5 w-5/6 rounded-sm mb-1"
              :style="{ background: t.colors.text, opacity: 0.2 }"
            />
            <div
              class="h-1.5 w-1/3 rounded-sm"
              :style="{ background: t.colors.accent, opacity: 0.7 }"
            />
          </div>
          <!-- Label -->
          <div class="text-xs font-medium text-gray-700 text-center">
            {{ t.label }}
          </div>
        </button>
      </div>
    </div>

    <!-- Dark themes -->
    <div>
      <h3 class="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-3">
        Dark
      </h3>
      <div class="grid grid-cols-3 gap-3">
        <button
          v-for="t in DARK_THEMES"
          :key="t.id"
          class="rounded-xl border-2 p-3 transition-all duration-150 cursor-pointer hover:shadow-md hover:-translate-y-0.5"
          :class="
            store.currentTheme === t.id
              ? 'border-blue-500 shadow-md'
              : 'border-gray-200 hover:border-gray-300'
          "
          @click="select(t.id)"
        >
          <div class="flex h-4 rounded-md overflow-hidden mb-2.5 shadow-sm">
            <span class="flex-1" :style="{ background: t.colors.bg }" />
            <span class="flex-1" :style="{ background: t.colors.text }" />
            <span class="flex-1" :style="{ background: t.colors.accent }" />
          </div>
          <div
            class="rounded-md p-2 mb-2"
            :style="{ background: t.colors.bg }"
          >
            <div
              class="h-2 w-3/4 rounded-sm mb-1.5"
              :style="{ background: t.colors.text, opacity: 0.8 }"
            />
            <div
              class="h-1.5 w-full rounded-sm mb-1"
              :style="{ background: t.colors.text, opacity: 0.2 }"
            />
            <div
              class="h-1.5 w-5/6 rounded-sm mb-1"
              :style="{ background: t.colors.text, opacity: 0.2 }"
            />
            <div
              class="h-1.5 w-1/3 rounded-sm"
              :style="{ background: t.colors.accent, opacity: 0.7 }"
            />
          </div>
          <div class="text-xs font-medium text-gray-700 text-center">
            {{ t.label }}
          </div>
        </button>
      </div>
    </div>
  </div>
</template>
