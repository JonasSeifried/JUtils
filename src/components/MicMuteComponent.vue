<script setup lang="ts">
import Hotkey from "./HotkeyComponent.vue";
import { HotkeyNames } from "../hotkey-names";

function hotkey_pressed() {
  console.log("test")
}
</script>

<template>
  <div class="mic-mute-component">
    <h1>MicMute</h1>
    <Hotkey class="hotkey-component" @callback="hotkey_pressed" :hotkey_name="HotkeyNames.MicMute" />
  </div>
</template>

<style scoped>
#error {
  color: lightcoral;
  margin: 0;
}

.hotkey-component {
  max-width: 20rem;
  align-self: center;
}

.mic-mute-component {
  display: flex;
  flex-direction: column;
  width: 100%;
}
</style>
