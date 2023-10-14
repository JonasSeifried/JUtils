<script setup lang="ts">
import { register, unregister } from "@tauri-apps/api/globalShortcut";
import { invoke } from "@tauri-apps/api";
import { computed, onMounted, ref } from "vue";

const probs = defineProps<{
  hotkey_name: string;
}>();
const emit = defineEmits<{
 (e: 'callback', value: void): void 
}>()

var current_hotkey: string | null;
var error_message = ref("");

var input_field: HTMLInputElement;

const error_disabled = computed(() => {
  return error_message.value.length == 0;
});

async function submit() {
  error_message.value = "";
  try {
    if (input_field.value.length == 0 && current_hotkey) {
      await unregister(current_hotkey);
      return;
    }

    await register(input_field.value, () => emit("callback"));

    if (current_hotkey) { unregister(current_hotkey); }
  } catch (error) {
    console.log(error);
    error_message.value = error as string;
    return
  }

  console.log("registered: " + probs.hotkey_name)
  current_hotkey = input_field.value;
  setHotKey()
}
function clear() {
  input_field.value = "";
}

function setHotKey() {
  invoke(`set_${probs.hotkey_name}_hotkey`, { keys: current_hotkey })
      .catch((error) => {
        error_message.value = "Could not save hotkey \n" + error;
      });
}

async function loadHotkey() {
  try {
    let res = await invoke(`fetch_${probs.hotkey_name}_hotkey`)
    await register(res as string, () => emit("callback"));
    if (input_field) {
        input_field.value = res as string;
        current_hotkey = res as string
    }
  } catch (error) {
    error_message.value = error as string
  }
}

onMounted(() => {
  input_field = document.getElementById("input_field") as HTMLInputElement;
  loadHotkey();
});
</script>

<template>
  <div class="hotkey-component">
    <p id="error" v-if="!error_disabled">{{ error_message }}</p>
    <div id="input-div">
      <input id="input_field" placeholder="hotkey" />
      <button id="clear-btn" @click="clear">Clear</button>
    </div>
    <button id="submit-btn" @click="submit">Save</button>
  </div>
</template>

<style scoped>
#error {
  color: lightcoral;
  margin: 0;
}

#input-div {
  margin: none;
  margin-bottom: 0.5rem;
  padding: none;
  border: 2px solid var(--primary);
  border-radius: 8px;

  width: fit-content;
  align-self: center;
}

#clear-btn {
  border: none;
  border-left: 2px solid var(--primary);
  border-radius: none;
  background-color: transparent;
}

#clear-btn:hover {
  box-shadow: 0px 0px 3px 4px var(--primary);
}

#input_field {
  border: none;
  background-color: transparent;
  height: 2rem;
}

#submit-btn {
  width: 5rem;
  align-self: center;
  color: var(--primary);
}


.hotkey-component {
  display: flex;
  flex-direction: column;
  width: 100%;
}
</style>
