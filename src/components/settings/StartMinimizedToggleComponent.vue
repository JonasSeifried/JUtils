<script setup lang="ts">
import { onMounted, ref } from "vue";
import ToggleComponent from "../ToggleComponent.vue";
import { debug, error } from "@tauri-apps/plugin-log";
import { settingsStore } from "../../store";

const initValue = ref(false);
const isFetching = ref(true);

function autoStartToggle(newValue: boolean) {
  settingsStore
    .set("start_minimized_state", newValue)
    .then(() => {
      debug(`start minimized -> ${newValue}`);
    })
    .catch((err) => {
      error(err);
    });
}

onMounted(() => {
  settingsStore
    .get<boolean>("start_minimized_state")
    .then((initState) => {
      if (initState === undefined) {
        initState = false; // Default value if not set
      }

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
