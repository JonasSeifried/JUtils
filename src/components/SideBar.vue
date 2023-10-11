<script setup lang="ts">
import { open } from '@tauri-apps/api/shell'

const emit = defineEmits<{
  (event: 'navToView', view: string): void
}>()

var selectedElement: HTMLElement | null = document.getElementById("home")
selectedElement?.classList.add("active")

function navigate(view: string) {
  if (selectedElement?.id == view) { return }
  selectedElement?.classList.remove("active")


  emit('navToView', view)


  selectedElement = document.getElementById(view)
  selectedElement?.classList.add("active")

}
</script>

<template>
  <div class="content">
    <div>
      <h3 style="user-select: none;">JUtils</h3>
      <p class="nav-item" id="home" @click="navigate('home')">Home</p>
      <p class="nav-item" id="settings" @click="navigate('settings')">Settings</p>
      <p class="nav-item" id="micmute" @click="navigate('micmute')">MicMute</p>
    </div>
    <a id="jutils-link" @click="open('https://github.com/JonasSeifried/JUtils')">JUtils0.1</a>

  </div>
</template>

<style scoped>
.content {
  display: flex;
  flex-direction: column !important;
  justify-content: space-between !important;
  border-right: 2px solid purple;
  border-radius: 10px;
  box-shadow: 10px 0 15px -10px purple;
  user-select: none;

  height: 100%;
  width: 150px;
}

.active {
  color: gray !important;
}

.nav-item {
  transition: text-shadow 0.3s ease;
  color: white;
  user-select: none;

}

.nav-item:hover {
  color: purple;
  text-shadow: 4px 4px 15px purple;
}

#jutils-link {
  color: white;
  text-decoration: underline;
  margin: 1rem;

}
</style>

