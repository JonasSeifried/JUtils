import {createApp} from 'vue';
import './style.css';
import App from './App.vue';
import { attachConsole } from "tauri-plugin-log-api";

attachConsole();

createApp(App).mount('#app');
