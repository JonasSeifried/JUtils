<script setup lang="ts">
import { onMounted, ref } from "vue";
import ToggleComponent from "../ToggleComponent.vue";
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";

const initValue = ref(false);
const isFetching = ref(true);

async function autoStartToggle(newValue: boolean) {
  if (newValue == (await isEnabled())) {
    return;
  } else if (newValue) {
    await enable();
  } else {
    await disable();
  }
  console.log(`autoStart -> ${newValue}`);
}

onMounted(async () => {
  initValue.value = await isEnabled();
  isFetching.value = false;
});
</script>

<template>
  <div>
    <ToggleComponent
      v-if="!isFetching"
      title="Auto Start"
      tool-tip="Should this app start on startup"
      @toggle="autoStartToggle"
      :init-value="initValue"
      class="m-5"
    />
    <svg v-else class="animate-spin"></svg>
  </div>
</template>
