<script setup lang="ts">
import { register, unregister } from "@tauri-apps/api/globalShortcut";
import SnakeBar from "./SnakeBar.vue";
import { invoke } from "@tauri-apps/api";
import { onMounted, ref } from "vue";
import { SnakeBarType } from "../snake-bar-type";
import { HotkeyNames } from "../hotkey-manager";

const probs = defineProps<{
  hotkey_name: HotkeyNames;
}>();
const emit = defineEmits<{
  (e: "callback", value: void): void;
}>();

var current_hotkey: string;

const snakeBarText = ref("");
const snakeBarOpen = ref(false);
const snakeBarType = ref(SnakeBarType.error);
const inputValue = ref("");

async function submit() {
  if (current_hotkey == inputValue.value) {
    if (current_hotkey.length == 0)
      setSnakeBar("Hotkey already cleared", SnakeBarType.success);
    else
      setSnakeBar(
        `'${current_hotkey}' is already set as your hotkey`,
        SnakeBarType.success,
      );

    return;
  }
  setHotKey();
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
    setSnakeBar("Hotkey cleared!", SnakeBarType.success);
    return;
  }
  setSnakeBar(`${current_hotkey} registered!`, SnakeBarType.success);
}

async function registerHotkey(hotkey: string): Promise<boolean> {
  try {
    await register(hotkey, () => emit("callback"));
    return true;
  } catch (error) {
    setSnakeBar(error as string, SnakeBarType.error);

    console.log(error);
    return false;
  }
}

async function unregisterHotkey(hotkey: string) {
  try {
    await unregister(hotkey);
  } catch (error) {
    setSnakeBar(error as string, SnakeBarType.error);
    console.error(error);
  }
}

async function storeHotkey(hotkey: string): Promise<boolean> {
  await invoke(`set_${probs.hotkey_name}_hotkey`, {
    keys: hotkey,
  }).catch((error) => {
    setSnakeBar("Could not save hotkey \n" + error, SnakeBarType.error);
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

function setSnakeBar(msg: string, type: SnakeBarType) {
  snakeBarText.value = msg;
  snakeBarType.value = type;
  snakeBarOpen.value = true;
}

onMounted(() => {
  loadHotkey();
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
      />
      <button
        class="bg-transparent p-2 text-center text-lg text-white shadow-white hover:shadow-xl"
        @click="clear"
      >
        Clear
      </button>
    </div>
    <button
      class="mt-4 rounded p-1 text-white ring-2 ring-fuchsia-700"
      @click="submit"
    >
      Save
    </button>

    <SnakeBar v-model:open="snakeBarOpen" :type="snakeBarType">
      {{ snakeBarText }}
    </SnakeBar>
  </div>
</template>
