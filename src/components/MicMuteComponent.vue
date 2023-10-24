<script setup lang="ts">
import Hotkey from "./HotkeyComponent.vue";
import { HotkeyNames } from "../hotkey-names";
import { invoke } from "@tauri-apps/api";

function hotkey_pressed() {
  //Todo Error handling
  invoke("toggle_mic").catch((err) => {
    console.log(err);
  });
}
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <h1 class="mt-10 text-center text-5xl font-bold text-white">MicMute</h1>
    <p class="m-1 text-white">Set a global hotkey to mute your default Mic</p>
    <Hotkey
      class="m-2 w-80"
      @callback="hotkey_pressed"
      :hotkey_name="HotkeyNames.MicMute"
    />
  </div>
</template>
