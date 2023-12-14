<script setup lang="ts">
import { onMounted, ref } from "vue";
import ToggleComponent from "../ToggleComponent.vue";
import { invoke } from "@tauri-apps/api";
import { debug, error } from "tauri-plugin-log-api";

const initValue = ref(false);
const isFetching = ref(true);

function autoStartToggle(newValue: boolean) {
  invoke("set_start_minimized_state", { newState: newValue })
    .then(() => debug(`autoStart -> ${newValue}`))
    .catch((err) => error(err));
}

onMounted(() => {
  invoke<boolean>("fetch_start_minimized_state")
    .then((initState) => {
      initValue.value = initState;
      isFetching.value = false;
    })
    .catch((err) => error(err));
});
</script>

<template>
  <div>
    <ToggleComponent
      v-if="!isFetching"
      title="Start minimized"
      tool-tip="Should this app start minimized"
      @toggle="autoStartToggle"
      :init-value="initValue"
      class="m-5"
    />
    <svg v-else class="animate-spin"></svg>
  </div>
</template>
