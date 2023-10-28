import { register} from "@tauri-apps/api/globalShortcut";

import { invoke } from "@tauri-apps/api";

export enum HotkeyNames {
	MicMute = 'mic_mute',
}


export const HotkeyFunctions = {
	[HotkeyNames.MicMute] : "toggle_mic"
}

export function registerHotkeys() {
	registerMicMute()
}


async function registerMicMute() {
	const hotkey = await invoke(`fetch_${HotkeyNames.MicMute}_hotkey`)
	.catch((err) => {
		console.error(err);
		return
	});

	register(hotkey as string, () => {
		invoke(HotkeyFunctions[HotkeyNames.MicMute])
	})
}