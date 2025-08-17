<script setup lang="ts">
import Hotkey from "./HotkeyComponent.vue";
import SnackBar from "./SnackBar.vue";
import { computed, ref } from "vue";
import { SnackBarType } from "../snack-bar-type.ts";
import { debug } from "@tauri-apps/plugin-log";
import { settingsStore } from "../store.ts";

const snackBarText = ref("");
const snackBarOpen = ref(false);
const snackBarType = ref(SnackBarType.error);
const micMuteSliderValue = ref(50);
loadMicMuteSilderValue();

const micMuteSliderValueString = computed(() => {
  return `${micMuteSliderValue.value ?? 0}%`;
});

function onMicMuteSliderChange() {
  settingsStore
    .set("mic_mute_audio_volume", micMuteSliderValue.value as number)
    .then(() => {
      debug(`Mic Mute Slider Value set to ${micMuteSliderValue.value}%`);
      setSnackBar(
        `Mic Mute Volume set to ${micMuteSliderValue.value}%`,
        SnackBarType.success
      );
    })
    .catch((err) => {
      console.error(err);
      setSnackBar(err);
    });

  settingsStore.save().catch((err) => {
    console.error(err);
    setSnackBar(err);
  });
}

async function loadMicMuteSilderValue() {
  let res = await settingsStore
    .get<number>("mic_mute_audio_volume")
    .catch((err) => {
      console.error(err);
      setSnackBar(err);
    });

  debug(`loaded Mic Mute Slider Value -> ${res}%`);

  micMuteSliderValue.value = res as number;
}

function setSnackBar(msg: string, type: SnackBarType = SnackBarType.error) {
  snackBarText.value = msg;
  snackBarType.value = type;
  snackBarOpen.value = true;
}
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div class="mb-5 mt-10">
      <h1 class="text-center text-5xl font-bold text-white">MicMute</h1>
      <p class="m-1 text-white">Set a global hotkey to mute your default Mic</p>
    </div>
    <div class="mb-2 flex items-center">
      <label
        for="mic-mute-slider"
        class="mr-2 block text-center font-medium text-white"
        >Volume: {{ micMuteSliderValueString }}</label
      >
      <input
        id="mic-mute-slider"
        min="0"
        max="100"
        step="1"
        type="range"
        class="h-2 cursor-pointer appearance-none rounded-lg bg-neutral-800 accent-fuchsia-700"
        v-model="micMuteSliderValue"
        @change="onMicMuteSliderChange"
      />
    </div>
    <Hotkey class="m-2 w-80" :hotkey_name="'MicMute'" />
  </div>
  <SnackBar v-model:open="snackBarOpen" :type="snackBarType">
    {{ snackBarText }}
  </SnackBar>
</template>
