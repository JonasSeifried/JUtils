<script setup lang="ts">
import { register, unregister } from "@tauri-apps/api/globalShortcut";
import { invoke } from "@tauri-apps/api";
import { computed, onMounted, ref } from "vue";

const probs = defineProps<{
  hotkey_name: string;
}>();
const emit = defineEmits<{
  (e: "callback", value: void): void;
}>();

var current_hotkey: string | null;
var error_message = ref("");
var hint_message = ref("");

const inputValue = ref("");

const error_disabled = computed(() => {
  return error_message.value.length == 0;
});

const hint_disabled = computed(() => {
  return hint_message.value.length == 0;
});

async function submit() {
  error_message.value = "";
  if (current_hotkey == inputValue.value) {
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
    await registerHotkey(current_hotkey);
  }
  await storeHotkey(current_hotkey);
  if (oldHotkey) {
    await unregisterHotkey(oldHotkey);
  }
  if (current_hotkey.length == 0) {
    hint_message.value = "Hotkey cleared!";
    return;
  }
  hint_message.value = "registered: " + inputValue.value;
}

async function registerHotkey(hotkey: string) {
  try {
    await register(hotkey, () => emit("callback"));
  } catch (error) {
    error_message.value = error as string;
    console.log(error);
  }
}

async function unregisterHotkey(hotkey: string) {
  try {
    await unregister(hotkey);
  } catch (error) {
    error_message.value = error as string;
    console.log(error);
  }
}

async function storeHotkey(hotkey: string) {
  await invoke(`set_${probs.hotkey_name}_hotkey`, {
    keys: hotkey,
  }).catch((error) => {
    error_message.value = "Could not save hotkey \n" + error;
  });
}

async function loadHotkey() {
  let res = await invoke(`fetch_${probs.hotkey_name}_hotkey`);
  current_hotkey = res as string;
  inputValue.value = current_hotkey;
  if (current_hotkey.length != 0) {
    registerHotkey(current_hotkey);
  }
}

onMounted(() => {
  loadHotkey();
});
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <p class="m-1 text-center text-red-400" v-if="!error_disabled">
      {{ error_message }}
    </p>
    <p class="m-1 text-center text-green-500" v-if="!hint_disabled">
      {{ hint_message }}
    </p>
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
  </div>
</template>
