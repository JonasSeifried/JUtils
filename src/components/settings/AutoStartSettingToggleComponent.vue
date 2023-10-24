<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";
import ToggleComponent from "../ToggleComponent.vue";

const initValue = ref(false);
const isFetching = ref(true);

invoke("get_auto_launch").then((value) => {
  initValue.value = value as boolean;
  isFetching.value = false;
});

function autoStartToggle(newValue: boolean) {
  setAutoLaunch(newValue);
}

function setAutoLaunch(newValue: boolean) {
  invoke("set_auto_launch", { newState: newValue });
}
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
    <svg v-else class="animate-spin" />
  </div>
</template>
