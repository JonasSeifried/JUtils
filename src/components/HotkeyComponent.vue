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
  try {
    if (inputValue.value.length == 0 && current_hotkey) {
      await unregister(current_hotkey);
      setHotKey();
      return;
    }
    if (inputValue.value == current_hotkey) {
      return;
    }

    await register(inputValue.value, () => emit("callback"));

    if (current_hotkey) {
      unregister(current_hotkey);
    }
  } catch (error) {
    console.log(error);
    error_message.value = error as string;
    return;
  }
  hint_message.value = "registered: " + inputValue.value;

  current_hotkey = inputValue.value;
  setHotKey();
}
function clear() {
  inputValue.value = "";
}

function setHotKey() {
  invoke(`set_${probs.hotkey_name}_hotkey`, { keys: current_hotkey }).catch(
    (error) => {
      error_message.value = "Could not save hotkey \n" + error;
    },
  );
}

async function loadHotkey() {
  try {
    let res = await invoke(`fetch_${probs.hotkey_name}_hotkey`);
    await register(res as string, () => emit("callback"));
    inputValue.value = res as string;
    current_hotkey = res as string;
  } catch (error) {
    error_message.value = error as string;
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
