<script setup lang="ts">
import { Component } from "vue";
const props = defineProps<{
  content: Component;
}>();
</script>

<template>
  <div>
    <div
      id="snackbar"
      class="fixed top-[75%] z-[999] flex h-full w-[inherit] flex-col items-center"
    ></div>
    <props.content />
  </div>
</template>
