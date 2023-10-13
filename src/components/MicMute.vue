<script setup lang="ts">
import { register, unregister } from '@tauri-apps/api/globalShortcut';
import { invoke } from "@tauri-apps/api"
import { onMounted } from 'vue';

var current_hotkey: string | null

var input_field: HTMLInputElement
var errorElement: HTMLParagraphElement

async function submit() {
    try {
        if (input_field.value.length == 0 && current_hotkey) {
            await unregister(current_hotkey)
            return
        }
        await register(input_field.value, (shortcut) => {
            console.log(`Shortcut ${shortcut} triggered`);
        })
        current_hotkey = input_field.value
        await invoke("set_mic_mute_hotkey", { keys: current_hotkey })
            .then((res) => {
                console.log(res)
            })
            .catch((error) => { errorElement.textContent = error })
    } catch (error) {
        console.log(error)
        errorElement.textContent = error as string
    }
}

function clear() {
    input_field.value = ""
}

function loadHotkey() {
    invoke("fetch_mic_mute_hotkey")
        .then((res) => {
            if (input_field) {
                current_hotkey = res as string
                input_field.value = res as string
            }
        })
        .catch((error) => {
            errorElement.textContent = error
        })
}

onMounted(() => {
    input_field = document.getElementById("input_field") as HTMLInputElement
    errorElement = document.getElementById("error") as HTMLParagraphElement
    loadHotkey()
})



</script>

<template >
    <div class="this">

        <h1>MicMute</h1>
        <p id="error"></p>
        <input id="input_field" placeholder="hotkey" />
        <button @click="clear">Clear</button>
        <button @click="submit">submit</button>
    </div>
</template>

<style scoped>
#error {
    color: lightcoral;
    margin: 0;
}

.this {
    display: flex;
    flex-direction: column;
    width: 100%;
}
</style>