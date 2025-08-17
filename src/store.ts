import { Store } from "@tauri-apps/plugin-store";

const store = await Store.load(".settings.json");

export const settingsStore = store;
