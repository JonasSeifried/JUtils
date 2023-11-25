<script setup lang="ts">
import SideBar from "./components/SideBarComponent.vue";
import Home from "./components/HomeComponent.vue";
import Settings from "./components/settings/SettingsComponent.vue";
import { shallowRef } from "vue";
import MicMute from "./components/MicMuteComponent.vue";
import Main from "./components/Main.vue";

const navigate = (view: string) => {
  switch (view) {
    case "home":
      currentView.value = Home;
      break;
    case "settings":
      currentView.value = Settings;
      break;
    case "micmute":
      currentView.value = MicMute;
      break;
    default:
      currentView.value = Home;
      break;
  }
};

const currentView = shallowRef(Home);
</script>

<template>
  <div class="flex h-full select-none flex-row justify-stretch bg-neutral-900">
    <SideBar @navToView="navigate" class="w-1/6" />
    <Main :content="currentView" class="w-5/6" />
  </div>
</template>
