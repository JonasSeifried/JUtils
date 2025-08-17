import { createApp } from "vue";
import App from "./App.vue";
import { loadAndRegisterShortcuts } from "./shortcuts";

createApp(App).mount("#app");

loadAndRegisterShortcuts();
