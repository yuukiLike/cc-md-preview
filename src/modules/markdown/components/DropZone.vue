<script setup lang="ts">
defineProps<{
  active: boolean;
}>();
</script>

<template>
  <Transition name="fade">
    <div
      v-if="active"
      class="absolute inset-0 z-50 flex items-center justify-center bg-gray-900/40 backdrop-blur-sm"
    >
      <div
        class="border-2 border-dashed border-white rounded-2xl px-12 py-8 text-white text-lg font-medium"
      >
        Drop .md file here
      </div>
    </div>
  </Transition>
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
