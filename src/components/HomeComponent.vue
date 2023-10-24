<script setup lang="ts">
import { open } from "@tauri-apps/api/shell";
</script>
<template>
  <div class="flex w-full flex-col items-center">
    <h1 class="mt-10 text-center text-5xl font-bold text-white">JUtils</h1>
    <a
      @click="open('https://github.com/JonasSeifried')"
      class="cursor-pointer whitespace-nowrap text-center text-fuchsia-700 hover:text-fuchsia-500"
      >by Jonas Seifried</a
    >
  </div>
</template>
