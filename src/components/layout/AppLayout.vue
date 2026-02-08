<script setup lang="ts">
import AppSidebar from "./AppSidebar.vue";
</script>

<template>
  <div class="flex h-screen bg-white">
    <AppSidebar />
    <main class="flex-1 overflow-auto">
      <RouterView />
    </main>
  </div>
</template>
