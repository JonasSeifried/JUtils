<script setup lang="ts">
import { register, unregister } from "@tauri-apps/api/globalShortcut";
import SnackBar from "./SnackBar.vue";
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";
import { SnackBarType } from "../snack-bar-type";
import { HotkeyNames } from "../hotkey-manager";

const probs = defineProps<{
  hotkey_name: HotkeyNames;
}>();
const emit = defineEmits<{
  (e: "callback", value: void): void;
}>();

var current_hotkey: string;

const snackBarText = ref("");
const snackBarOpen = ref(false);
const snackBarType = ref(SnackBarType.error);
const inputValue = ref("");
var hotkeyIsBeingEdited = false;

async function submit() {
  if (current_hotkey == inputValue.value) {
    if (current_hotkey.length == 0)
      setSnackBar("Hotkey already cleared", SnackBarType.success);
    else
      setSnackBar(
        `'${current_hotkey}' is already set as your hotkey`,
        SnackBarType.success,
      );

    return;
  }

  invoke("set_mic_mute_hotkey", { keys: inputValue.value })
    .then(() => {
      current_hotkey = inputValue.value;
      setSnackBar(`${current_hotkey} registered!`, SnackBarType.success);
    })
    .catch((error) => {
      setSnackBar(error as string, SnackBarType.error);
    });
}

function clear() {
  inputValue.value = "";
}

async function setHotKey() {
  const oldHotkey = current_hotkey;
  current_hotkey = inputValue.value;
  if (current_hotkey.length != 0) {
    if (!(await registerHotkey(current_hotkey))) {
      current_hotkey = oldHotkey;
      return;
    }
  }
  if (!(await storeHotkey(current_hotkey))) {
    unregister(current_hotkey);
    current_hotkey = oldHotkey;
    return;
  }
  unregisterHotkey(oldHotkey);
  if (current_hotkey.length == 0) {
    setSnackBar("Hotkey cleared!", SnackBarType.success);
    return;
  }
  setSnackBar(`${current_hotkey} registered!`, SnackBarType.success);
}

async function registerHotkey(hotkey: string): Promise<boolean> {
  try {
    await register(hotkey, () => emit("callback"));
    return true;
  } catch (error) {
    setSnackBar(error as string, SnackBarType.error);

    console.log(error);
    return false;
  }
}

async function unregisterHotkey(hotkey: string) {
  try {
    await unregister(hotkey);
  } catch (error) {
    setSnackBar(error as string, SnackBarType.error);
    console.error(error);
  }
}

async function storeHotkey(hotkey: string): Promise<boolean> {
  await invoke(`set_${probs.hotkey_name}_hotkey`, {
    keys: hotkey,
  }).catch((error) => {
    setSnackBar("Could not save hotkey \n" + error, SnackBarType.error);
    console.error(error);

    return false;
  });
  return true;
}

async function loadHotkey() {
  let res = await invoke(`fetch_${probs.hotkey_name}_hotkey`);
  current_hotkey = res as string;
  inputValue.value = current_hotkey;
}

function inputKeyDown(payload: KeyboardEvent) {
  payload.preventDefault();
  if (payload.repeat) return;
  console.log(payload);
  const key = payload.key;
  if (!hotkeyIsBeingEdited) {
    inputValue.value = "";

    hotkeyIsBeingEdited = true;
  }
  inputValue.value += key;

  if (key === "Control" || key === "Alt" || key === "Shift") {
    inputValue.value += "+";
  } else {
  }
}
function inputKeyUp() {
  hotkeyIsBeingEdited = false;
}

function setSnackBar(msg: string, type: SnackBarType = SnackBarType.error) {
  snackBarText.value = msg;
  snackBarType.value = type;
  snackBarOpen.value = true;
}

onMounted(async () => {
  loadHotkey();
  console.log("test");
});
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div
      class="pointer-events-auto w-fit divide-x-2 divide-fuchsia-700 rounded-lg bg-neutral-800 p-0 outline-fuchsia-600 ring-2 ring-fuchsia-700 transition-transform focus-within:hover:scale-105"
    >
      <input
        v-model="inputValue"
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
