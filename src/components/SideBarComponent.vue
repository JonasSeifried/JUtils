<script setup lang="ts">
import { open } from "@tauri-apps/api/shell";
import { ref } from "vue";

const emit = defineEmits<{
  (event: "navToView", view: string): void;
}>();

var selectedElement: HTMLElement | null = document.getElementById("home");
selectedElement?.classList.add("active");

function navigate(view: string) {
  if (selectedElement?.id == view) {
    return;
  }
  selectedElement?.classList.remove("text-fuchsia-700");
  selectedElement?.classList.add("text-white");

  emit("navToView", view);

  selectedElement = document.getElementById(view);
  selectedElement?.classList.add("text-fuchsia-700");
  selectedElement?.classList.remove("text-white");
}
const navigations = ref([
  { id: "home", title: "Home" },
  { id: "settings", title: "Settings" },
  { id: "micmute", title: "MicMute" },
]);
</script>

<template>
  <div
    class="flex h-full cursor-pointer flex-col justify-between rounded-xl border-r-2 border-solid border-fuchsia-700 text-center shadow-r-2xl shadow-fuchsia-700"
  >
    <div>
      <h3 class="m-6 cursor-default text-2xl text-white">JUtils</h3>
      <div v-for="nav in navigations">
        <p
          class="m-3 text-white shadow-fuchsia-950 transition-all hover:scale-105 hover:text-fuchsia-700 hover:text-shadow"
          :id="nav.id"
          @click="navigate(nav.id)"
        >
          {{ nav.title }}
        </p>
      </div>
    </div>
    <a
      class="hover: m-4 text-white underline transition-all hover:scale-105"
      @click="open('https://github.com/JonasSeifried/JUtils')"
      >JUtils0.1</a
    >
  </div>
</template>
