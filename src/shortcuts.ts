// src/lib/hotkeys.ts
import {
  register,
  unregisterAll,
  unregister,
  ShortcutHandler,
} from "@tauri-apps/plugin-global-shortcut";
import { settingsStore } from "./store";
import { error, warn } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

// Define a type for your shortcuts
type ShortcutMap = {
  [actionName: string]: {
    shortcut: string;
    action: ShortcutHandler;
  };
};

const store = settingsStore;

// shortcuts with default values
export const shortcuts: ShortcutMap = {
  hotkey_MicMute: {
    shortcut: "CmdOrControl+1",
    action: async (event) => {
      if (event.state !== "Pressed") {
        return; // Only handle keydown events
      }
      invoke("toggle_mic");
    },
  },
};

/**
 * Loads user-defined shortcuts from the store and registers them.
 */
export async function loadAndRegisterShortcuts(): Promise<void> {
  await unregisterAll(); // Start with a clean slate

  for (const actionName of Object.keys(shortcuts)) {
    const storedShortcut = await store.get<string>(actionName);
    const shortcutString = storedShortcut || shortcuts[actionName].shortcut;

    try {
      await register(shortcutString, shortcuts[actionName].action);
    } catch (err) {
      error(
        `Failed to register shortcut "${shortcutString}" for action "${actionName}": ${err}`
      );
    }
  }
}

/**
 * Updates a shortcut for a specific action and saves it to the store.
 * @param actionName The name of the action (e.g., 'save-file').
 * @param newShortcut The new shortcut string (e.g., 'Ctrl+Shift+S').
 */
export async function updateShortcut(
  actionName: string,
  newShortcut: string
): Promise<void> {
  const oldShortcut =
    (await store.get<string>(actionName)) || shortcuts[actionName].shortcut;

  if (oldShortcut) {
    await unregister(oldShortcut).catch((err) => {
      warn(
        `Failed to unregister old shortcut "${oldShortcut}" for action "${actionName}": ${err}`
      );
    });
  }

  await register(newShortcut, shortcuts[actionName].action);
  await store.set(actionName, newShortcut);
  await store.save();
}
