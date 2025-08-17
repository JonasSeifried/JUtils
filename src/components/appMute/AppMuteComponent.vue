<script setup lang="ts">
import Hotkey from "../HotkeyComponent.vue";
import Search, { SearchItem } from "../SearchComponent.vue";
import SnackBar from "../SnackBar.vue";
import { computed, onMounted, ref } from "vue";
import { SnackBarType } from "../../snack-bar-type.ts";
import { debug, error } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

const snackBarText = ref("");
const snackBarOpen = ref(false);
const snackBarType = ref(SnackBarType.error);
const selectedApp = ref("");
const hotkeyString = ref("");
const apps = ref<SearchItem[]>([]);

function setSnackBar(msg: string, type: SnackBarType = SnackBarType.error) {
  snackBarText.value = msg;
  snackBarType.value = type;
  snackBarOpen.value = true;
}

function onAppSelected(app: SearchItem) {
  selectedApp.value = app.displayName;
  hotkeyString.value = `app_mute_${app.id}`;
  debug(`Selected app: ${app.displayName}`);
  invoke("toggle_app_mute", { appName: app.id })
    .then(() => {
      setSnackBar(`Toggled mute for ${app.displayName}`, SnackBarType.success);
    })
    .catch((err) => {
      error(`Error toggling app mute: ${err}`);
      setSnackBar(err as string, SnackBarType.error);
    });
}

function mapAppToSearchItem(app: string): SearchItem {
  switch (app) {
    case "@%SystemRoot%\\System32\\AudioSrv.Dll,-202":
      return {
        id: app,
        displayName: "System",
      };
    default:
      return { id: app, displayName: app.replace(".exe", "") };
  }
}

onMounted(async () => {
  invoke<string[]>("get_running_apps")
    .then((res) => {
      apps.value = res.map(mapAppToSearchItem);
    })

    .catch((err) => {
      error(`Error fetching running apps: ${err}`);
      setSnackBar(err as string, SnackBarType.error);
    });
});
</script>

<template>
  <div class="flex w-full flex-col items-center">
    <div class="mb-5 mt-10">
      <h1 class="text-center text-5xl font-bold text-white">App Mute</h1>
      <p class="m-1 text-white">Set a global hotkeys to mute apps</p>
    </div>

    <Search :items="apps" v-on:item-selected="onAppSelected" />
    {{ selectedApp }}
    <Hotkey class="m-2 w-80" :hotkey_name="hotkeyString" v-if="hotkeyString" />
  </div>
  <SnackBar v-model:open="snackBarOpen" :type="snackBarType">
    {{ snackBarText }}
  </SnackBar>
</template>
