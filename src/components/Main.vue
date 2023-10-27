<script setup lang="ts">
import { Component } from "vue";

const props = defineProps<{
  content: Component;
}>();
</script>

<template>
  <div>
    <div id="snackbar" class="w-[inherit]"></div>
    <props.content />
  </div>
</template>
