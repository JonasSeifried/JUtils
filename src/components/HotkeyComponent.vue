<script setup lang="ts">
import SnackBar from "./SnackBar.vue";
import { computed, onMounted, ref, Ref } from "vue";
import { SnackBarType } from "../snack-bar-type.ts";
import { error, info } from "@tauri-apps/plugin-log";
import { updateShortcut } from "../shortcuts.ts";
import { settingsStore } from "../store.ts";

const probs = defineProps<{
  hotkey_name: string;
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
        SnackBarType.success
      );
    return;
  }
  updateShortcut(`hotkey_${probs.hotkey_name}`, hotkeyString.value)
    .then(() => {
      info(`hotkey for ${probs.hotkey_name} updated to: ${hotkeyString.value}`);
      inputHasChanged = false;
      setSnackBar(
        keys.value.size == 0
          ? "hotkey cleared"
          : `${hotkeyString.value} registered!`,
        SnackBarType.success
      );
    })
    .catch((err) => {
      error(err);
      setSnackBar(err as string, SnackBarType.error);
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
  inputHasChanged = true;
  if (!hotkeyIsBeingEdited) {
    keys.value.clear();
    hotkeyIsBeingEdited = true;
  }

  keys.value.set(payload.code, payload.key.toLowerCase());
}
function inputKeyUp() {
  hotkeyIsBeingEdited = false;
}

const hotkeyString = computed(() => {
  return Array.from(keys.value.values()).join("+");
});

function setSnackBar(msg: string, type: SnackBarType = SnackBarType.error) {
  snackBarText.value = msg;
  snackBarType.value = type;
  snackBarOpen.value = true;
}

onMounted(() => {
  settingsStore
    .get<string>(`hotkey_${probs.hotkey_name}`)
    .then((hotkey: string | undefined) => {
      if (hotkey) {
        hotkey.split("+").forEach((key) => keys.value.set(key, key));
      }
      info("loaded hotkey: " + hotkey);
    })
    .catch((err) => {
      error(err);
      setSnackBar(err as string, SnackBarType.error);
    });
});
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div
      class="pointer-events-auto w-fit divide-x-2 divide-fuchsia-700 rounded-lg bg-neutral-800 p-0 outline-fuchsia-600 ring-2 ring-fuchsia-700 transition-transform focus-within:hover:scale-105"
    >
      <input
        v-model="hotkeyString"
        class="bg-transparent p-2 text-lg text-text focus:outline-none"
        placeholder="No active hotkey"
        spellcheck="false"
        @keydown="inputKeyDown"
        @keyup="inputKeyUp"
      />
      <button
        class="rounded-[inherit] bg-transparent p-2 text-center text-lg text-text hover:bg-neutral-900"
        @click="clear"
      >
        Clear
      </button>
    </div>
    <button
      class="mt-4 rounded-lg bg-neutral-800 p-1 text-lg text-text ring-2 ring-fuchsia-700 hover:bg-neutral-900"
      @click="submit"
    >
      Save
    </button>

    <SnackBar v-model:open="snackBarOpen" :type="snackBarType">
      {{ snackBarText }}
    </SnackBar>
  </div>
</template>
