<script setup lang="ts">
import Hotkey from "./HotkeyComponent.vue";
import SnakeBar from "./SnakeBar.vue";
import { HotkeyNames } from "../hotkey-manager";
import { invoke } from "@tauri-apps/api";
import { computed, ref } from "vue";
import { SnakeBarType } from "../snake-bar-type";

const snakeBarText = ref("");
const snakeBarOpen = ref(false);
const snakeBarType = ref(SnakeBarType.error);
const micMuteSliderValue = ref(50);
loadMicMuteSilderValue();

const micMuteSliderValueString = computed(() => {
  return `${micMuteSliderValue.value}%`;
});

function onMicMuteSliderChange(payload: Event) {
  invoke("set_mic_mute_audio_volume", {
    volume: +(payload.target as HTMLInputElement).value,
  }).catch((err) => {
    console.error(err);
    setSnakeBar(err);
  });
}

async function loadMicMuteSilderValue() {
  let res = await invoke("get_mic_mute_audio_volume").catch((err) => {
    console.error(err);
    setSnakeBar(err);
    return;
  });
  console.log(`loaded Mic Mute Slider Value -> ${res}%`);

  micMuteSliderValue.value = res as number;
}

function setSnakeBar(msg: string, type: SnakeBarType = SnakeBarType.error) {
  snakeBarText.value = msg;
  snakeBarType.value = type;
  snakeBarOpen.value = true;
}

function hotkey_pressed() {
  invoke("toggle_mic").catch((err) => {
    console.error(err);
    setSnakeBar(err);
  });
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
    <Hotkey
      class="m-2 w-80"
      @callback="hotkey_pressed"
      :hotkey_name="HotkeyNames.MicMute"
    />
  </div>
  <SnakeBar v-model:open="snakeBarOpen" :type="snakeBarType">
    {{ snakeBarText }}
  </SnakeBar>
</template>
