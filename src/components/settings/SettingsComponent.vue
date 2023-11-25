<script setup lang="ts">
import AutoStartSettingToggleComponent from "./AutoStartSettingToggleComponent.vue";
import StartMinimizedToggleComponent from "./StartMinimizedToggleComponent.vue";
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div class="mb-5 mt-10">
      <h1 class="text-center text-5xl font-bold text-white">Settings</h1>
      <p class="m-1 text-white">Customize Settings to suit your preferences</p>
    </div>

    <div>
      <AutoStartSettingToggleComponent />
      <StartMinimizedToggleComponent />
    </div>
  </div>
</template>
