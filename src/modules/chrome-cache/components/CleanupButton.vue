<script setup lang="ts">
import { ref } from "vue";
import type { CacheCategory } from "../types";
import { formatBytes } from "../utils";

const props = defineProps<{
  categories: CacheCategory[];
  isCleaning: boolean;
}>();

const emit = defineEmits<{
  clean: [types: string[]];
}>();

const showConfirm = ref(false);

function totalSize(): number {
  return props.categories
    .filter((c) => c.exists)
    .reduce((sum, c) => sum + c.size, 0);
}

function existingTypes(): string[] {
  return props.categories.filter((c) => c.exists && c.size > 0).map((c) => c.name);
}

function confirm() {
  showConfirm.value = false;
  emit("clean", existingTypes());
}
</script>

<template>
  <div class="pt-2">
    <button
      class="w-full py-2.5 text-sm font-medium rounded-lg transition-colors disabled:opacity-50"
      :class="
        isCleaning
          ? 'bg-gray-200 text-gray-500'
          : 'bg-red-50 text-red-600 hover:bg-red-100 border border-red-200'
      "
      :disabled="isCleaning || existingTypes().length === 0"
      @click="showConfirm = true"
    >
      {{ isCleaning ? "Cleaning..." : "Clean All Cache" }}
    </button>

    <Teleport to="body">
      <Transition name="fade">
        <div
          v-if="showConfirm"
          class="fixed inset-0 z-50 flex items-center justify-center bg-black/40"
          @click.self="showConfirm = false"
        >
          <div
            class="bg-white rounded-xl shadow-lg p-6 max-w-sm w-full mx-4 space-y-4"
          >
            <h3 class="text-base font-semibold text-gray-900">
              Clean Chrome Cache?
            </h3>
            <p class="text-sm text-gray-600">
              This will free approximately
              <strong>{{ formatBytes(totalSize()) }}</strong
              >. This action cannot be undone.
            </p>
            <div class="flex gap-3 justify-end">
              <button
                class="px-4 py-2 text-sm rounded-lg bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors"
                @click="showConfirm = false"
              >
                Cancel
              </button>
              <button
                class="px-4 py-2 text-sm rounded-lg bg-red-600 text-white hover:bg-red-700 transition-colors"
                @click="confirm"
              >
                Clean
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
