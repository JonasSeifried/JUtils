<script setup lang="ts">
import SideBar from "./components/SideBar.vue"
import Home from "./components/Home.vue"
import Settings from "./components/Settings.vue"
import { shallowRef } from "vue"
import MicMute from "./components/MicMute.vue";


const navigate = (view: string) => {
  switch (view) {
    case "home":
      currentView.value = Home
      break
    case "settings":
      currentView.value = Settings
      break
    case "micmute":
      currentView.value = MicMute
      break
    default:
      currentView.value = Home
      break
  }
}

const currentView = shallowRef(Home)
</script>

<template>
  <div class=content>
    <SideBar @navToView="navigate" />
    <currentView />
  </div>
</template>

<style scoped>
.content {
  display: flex;
  flex-direction: row;
  justify-content: start;
  height: 100%;


}
</style>
