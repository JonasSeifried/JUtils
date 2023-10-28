import {createApp} from 'vue';
import './style.css';
import App from './App.vue';
import { registerHotkeys } from "./hotkey-manager";

registerHotkeys();
createApp(App).mount('#app');
