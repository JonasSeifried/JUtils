<script setup lang="ts">
import SnackBar from "./SnackBar.vue";
import { invoke } from "@tauri-apps/api";
import { computed, ref, Ref } from "vue";
import { SnackBarType } from "../snack-bar-type";
import { HotkeyNames } from "../hotkey-manager";

const probs = defineProps<{
  hotkey_name: HotkeyNames;
}>();

const keys: Ref<Map<string, string>> = ref(new Map());
const snackBarText = ref("");
const snackBarOpen = ref(false);
const snackBarType = ref(SnackBarType.error);
var hotkeyIsBeingEdited = false;
var inputHasChanged = false;

async function submit() {
  if (!inputHasChanged) {
    if (hotkeyString.value.length == 0)
      setSnackBar("Hotkey already cleared", SnackBarType.success);
    else
      setSnackBar(
        `'${hotkeyString.value}' is already set as your hotkey`,
        SnackBarType.success,
      );
    return;
  }
  invoke(`set_${probs.hotkey_name}_hotkey`, {
    keys: Array.from(keys.value.keys()),
  })
    .then(() => {
      inputHasChanged = false;
      let msg =
        keys.value.size == 0
          ? "hotkey cleared"
          : `${hotkeyString.value} registered!`;
      setSnackBar(msg, SnackBarType.success);
    })
    .catch((error) => {
      setSnackBar(error as string, SnackBarType.error);
    });
}

function clear() {
  if (keys.value.size != 0) {
    inputHasChanged = true;
    keys.value.clear();
  }
}

function inputKeyDown(payload: KeyboardEvent) {
  payload.preventDefault();
  if (payload.repeat) return;
  console.log(payload);
  inputHasChanged = true;
  if (!hotkeyIsBeingEdited) {
    keys.value.clear();
    hotkeyIsBeingEdited = true;
  }

  keys.value.set(payload.code, payload.key.toLowerCase());
  console.log(keys);
}
function inputKeyUp() {
  hotkeyIsBeingEdited = false;
}

const hotkeyString = computed(() => {
  console.log(Array.from(keys.value.keys()).join("+"));
  return Array.from(keys.value.values()).join("+");
});

function setSnackBar(msg: string, type: SnackBarType = SnackBarType.error) {
  snackBarText.value = msg;
  snackBarType.value = type;
  snackBarOpen.value = true;
}
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div
      class="pointer-events-auto w-fit divide-x-2 divide-fuchsia-700 rounded-lg bg-neutral-800 p-0 outline-fuchsia-600 ring-2 ring-fuchsia-700 transition-transform focus-within:hover:scale-105"
    >
      <input
        v-model="hotkeyString"
        class="bg-transparent p-2 text-lg text-white focus:outline-none"
        placeholder="No active hotkey"
        spellcheck="false"
        @keydown="inputKeyDown"
        @keyup="inputKeyUp"
      />
      <button
        class="rounded-[inherit] bg-transparent p-2 text-center text-lg text-white hover:bg-neutral-900"
        @click="clear"
      >
        Clear
      </button>
    </div>
    <button
      class="mt-4 rounded-lg bg-neutral-800 p-1 text-lg text-white ring-2 ring-fuchsia-700 hover:bg-neutral-900"
      @click="submit"
    >
      Save
    </button>

    <SnackBar v-model:open="snackBarOpen" :type="snackBarType">
      {{ snackBarText }}
    </SnackBar>
  </div>
</template>
