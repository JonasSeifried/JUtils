use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use crate::error::{Error, Result};

use global_hotkey::{
    hotkey::{Code, HotKey as GHotkey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use tauri::Manager;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

type HotKeyId = u32;

pub const MICMUTE: &str = "MicMute";

pub struct HotKeyManager {
    manager: GlobalHotKeyManager,
    hotkeys: Arc<Mutex<HashMap<HotKeyId, RegisteredHotkey>>>,
    required_hotkeys: Arc<Mutex<HashMap<String, Vec<HotKeyId>>>>,
    active_hotkeys: Arc<Mutex<Vec<HotKeyId>>>,
}

impl HotKeyManager {
    pub fn new() -> Self {
        HotKeyManager {
            manager: GlobalHotKeyManager::new().expect("Failed to init GlobalHotKeyManager"),
            hotkeys: Arc::new(Mutex::new(HashMap::<HotKeyId, RegisteredHotkey>::new())),
            required_hotkeys: Arc::new(Mutex::new(HashMap::<String, Vec<HotKeyId>>::new())),
            active_hotkeys: Arc::new(Mutex::new(Vec::<HotKeyId>::new())),
        }
    }

    pub fn register_hotkey(&self, name: &str, keys: Vec<String>) -> Result<()> {
        let mut required_hotkeys = self.required_hotkeys.lock().unwrap();
        self.unregister_hotkey(name, &mut required_hotkeys)?;
        if keys.len() == 0 {
            return Ok(());
        }

        let hotkeys = parse_hotkey(keys)?;
        for hk in hotkeys.clone() {
            self.manager.register(hk)?;
            self.hotkeys.lock().unwrap().insert(
                hk.id(),
                RegisteredHotkey {
                    name: name.into(),
                    hotkey: hk,
                },
            );
        }
        required_hotkeys.insert(name.into(), hotkeys.iter().map(|hk| hk.id()).collect());
        Ok(())
    }

    fn unregister_hotkey(
        &self,
        name: &str,
        required_hotkeys: &mut MutexGuard<'_, HashMap<String, Vec<HotKeyId>>>,
    ) -> Result<()> {
        let mut hotkeys: Vec<GHotkey> = Vec::new();
        if let Some(hotkey_ids) = required_hotkeys.get(name) {
            for hk_id in hotkey_ids {
                hotkeys.push(
                    self.hotkeys
                        .lock()
                        .unwrap()
                        .get(hk_id)
                        .ok_or(Error::UnexpectedError(format!(
                            "hotkeys should contain hotkey: {}",
                            hk_id
                        )))?
                        .hotkey,
                )
            }
        }
        if hotkeys.len() == 0 {
            return Ok(());
        }
        self.manager.unregister_all(&hotkeys)?;
        required_hotkeys
            .remove(name)
            .expect("registed hotkeys should be containted in required_hotkeys");
        Ok(())
    }
}

fn parse_hotkey(keys: Vec<String>) -> Result<Vec<GHotkey>> {
    let mut hotkeys: Vec<GHotkey> = Vec::new();
    let mut codes: Vec<Code> = Vec::new();
    let mut mods = Modifiers::empty();
    for key in keys {
        match key.as_str() {
            "ControlLeft" | "ControlRight" => mods.set(Modifiers::CONTROL, true),
            "ShiftLeft" | "ShiftRight" => mods.set(Modifiers::SHIFT, true),
            "MetaLeft" | "MetaRight" => mods.set(Modifiers::META, true),
            "AltLeft" => mods.set(Modifiers::ALT, true),
            "AltRight" => mods.set(Modifiers::ALT_GRAPH, true),
            "CapsLock" => mods.set(Modifiers::CAPS_LOCK, true),
            "Command" | "Super" => mods.set(Modifiers::SUPER, true),
            _ => codes.push(parse_key(key.as_str())?),
        };
    }
    for code in codes {
        match mods.is_empty() {
            true => hotkeys.push(GHotkey::new(None, code)),
            false => hotkeys.push(GHotkey::new(Some(mods), code)),
        }
    }
    Ok(hotkeys)
}

#[derive(Debug)]
pub struct RegisteredHotkey {
    pub name: String,
    pub hotkey: GHotkey,
}

#[derive(Debug)]
pub struct Hotkey {
    pub name: String,
    pub keys: Vec<String>,
}

pub fn init(hotkey_manager: &HotKeyManager, app: &mut tauri::App) {
    load_hotkeys(hotkey_manager);
    event_loop(hotkey_manager, app);
}

fn event_loop(hotkey_manager: &HotKeyManager, app: &mut tauri::App) {
    let receiver = GlobalHotKeyEvent::receiver();
    let event_loop = EventLoopBuilder::new().build().unwrap();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Wait);

            if let Ok(event) = receiver.try_recv() {
                if let Some(registered_hotkey) =
                    hotkey_manager.hotkeys.lock().unwrap().get(&event.id)
                {
                    if event.state == HotKeyState::Released {
                        hotkey_manager
                            .active_hotkeys
                            .lock()
                            .unwrap()
                            .retain(|id| id != &registered_hotkey.hotkey.id());
                        return;
                    }

                    hotkey_manager
                        .active_hotkeys
                        .lock()
                        .unwrap()
                        .push(registered_hotkey.hotkey.id());

                    let all_pressed = hotkey_manager
                        .required_hotkeys
                        .lock()
                        .unwrap()
                        .get(&registered_hotkey.name)
                        .expect("All hotkeys must be added to required_hotkeys")
                        .iter()
                        .all(|id| hotkey_manager.active_hotkeys.lock().unwrap().contains(id));
                    if all_pressed {
                        let res = match registered_hotkey.name.as_str() {
                            MICMUTE => crate::commands::toggle_mic(),
                            _ => Err(Error::UnexpectedError(format!(
                                "Hotkey name {} did not match",
                                registered_hotkey.name
                            ))),
                        };
                        if let Err(err) = res {
                            let _ = app.handle().emit_all("backend-error", &err);
                        }
                    }
                }
            }
        })
        .expect("Failed to start GlobalHotkey event loop");
}

fn load_hotkeys(hotkey_manager: &HotKeyManager) {
    load_hotkey(hotkey_manager, MICMUTE)
}

fn load_hotkey(hotkey_manager: &HotKeyManager, name: &str) {
    hotkey_manager
        .register_hotkey(
            name,
            crate::db::fetch_hotkey(name)
                .expect(format!("Failed to load hotkey {}", name).as_str())
                .keys,
        )
        .expect(format!("Failed to register hotkey {}", name).as_str())
}
fn parse_key(key: &str) -> Result<Code> {
    use Code::*;
    match key.to_uppercase().as_str() {
        "BACKQUOTE" | "`" => Ok(Backquote),
        "BACKSLASH" | "\\" => Ok(Backslash),
        "BRACKETLEFT" | "[" => Ok(BracketLeft),
        "BRACKETRIGHT" | "]" => Ok(BracketRight),
        "COMMA" | "," => Ok(Comma),
        "DIGIT0" | "0" => Ok(Digit0),
        "DIGIT1" | "1" => Ok(Digit1),
        "DIGIT2" | "2" => Ok(Digit2),
        "DIGIT3" | "3" => Ok(Digit3),
        "DIGIT4" | "4" => Ok(Digit4),
        "DIGIT5" | "5" => Ok(Digit5),
        "DIGIT6" | "6" => Ok(Digit6),
        "DIGIT7" | "7" => Ok(Digit7),
        "DIGIT8" | "8" => Ok(Digit8),
        "DIGIT9" | "9" => Ok(Digit9),
        "EQUAL" | "=" => Ok(Equal),
        "KEYA" | "A" => Ok(KeyA),
        "KEYB" | "B" => Ok(KeyB),
        "KEYC" | "C" => Ok(KeyC),
        "KEYD" | "D" => Ok(KeyD),
        "KEYE" | "E" => Ok(KeyE),
        "KEYF" | "F" => Ok(KeyF),
        "KEYG" | "G" => Ok(KeyG),
        "KEYH" | "H" => Ok(KeyH),
        "KEYI" | "I" => Ok(KeyI),
        "KEYJ" | "J" => Ok(KeyJ),
        "KEYK" | "K" => Ok(KeyK),
        "KEYL" | "L" => Ok(KeyL),
        "KEYM" | "M" => Ok(KeyM),
        "KEYN" | "N" => Ok(KeyN),
        "KEYO" | "O" => Ok(KeyO),
        "KEYP" | "P" => Ok(KeyP),
        "KEYQ" | "Q" => Ok(KeyQ),
        "KEYR" | "R" => Ok(KeyR),
        "KEYS" | "S" => Ok(KeyS),
        "KEYT" | "T" => Ok(KeyT),
        "KEYU" | "U" => Ok(KeyU),
        "KEYV" | "V" => Ok(KeyV),
        "KEYW" | "W" => Ok(KeyW),
        "KEYX" | "X" => Ok(KeyX),
        "KEYY" | "Y" => Ok(KeyY),
        "KEYZ" | "Z" => Ok(KeyZ),
        "MINUS" | "-" => Ok(Minus),
        "PERIOD" | "." => Ok(Period),
        "QUOTE" | "'" => Ok(Quote),
        "SEMICOLON" | ";" => Ok(Semicolon),
        "SLASH" | "/" => Ok(Slash),
        "BACKSPACE" => Ok(Backspace),
        "CAPSLOCK" => Ok(CapsLock),
        "ENTER" => Ok(Enter),
        "SPACE" => Ok(Space),
        "TAB" => Ok(Tab),
        "DELETE" => Ok(Delete),
        "END" => Ok(End),
        "HOME" => Ok(Home),
        "INSERT" => Ok(Insert),
        "PAGEDOWN" => Ok(PageDown),
        "PAGEUP" => Ok(PageUp),
        "PRINTSCREEN" => Ok(PrintScreen),
        "SCROLLLOCK" => Ok(ScrollLock),
        "ARROWDOWN" | "DOWN" => Ok(ArrowDown),
        "ARROWLEFT" | "LEFT" => Ok(ArrowLeft),
        "ARROWRIGHT" | "RIGHT" => Ok(ArrowRight),
        "ARROWUP" | "UP" => Ok(ArrowUp),
        "NUMLOCK" => Ok(NumLock),
        "NUMPAD0" | "NUM0" => Ok(Numpad0),
        "NUMPAD1" | "NUM1" => Ok(Numpad1),
        "NUMPAD2" | "NUM2" => Ok(Numpad2),
        "NUMPAD3" | "NUM3" => Ok(Numpad3),
        "NUMPAD4" | "NUM4" => Ok(Numpad4),
        "NUMPAD5" | "NUM5" => Ok(Numpad5),
        "NUMPAD6" | "NUM6" => Ok(Numpad6),
        "NUMPAD7" | "NUM7" => Ok(Numpad7),
        "NUMPAD8" | "NUM8" => Ok(Numpad8),
        "NUMPAD9" | "NUM9" => Ok(Numpad9),
        "NUMPADADD" | "NUMADD" | "NUMPADPLUS" | "NUMPLUS" => Ok(NumpadAdd),
        "NUMPADDECIMAL" | "NUMDECIMAL" => Ok(NumpadDecimal),
        "NUMPADDIVIDE" | "NUMDIVIDE" => Ok(NumpadDivide),
        "NUMPADENTER" | "NUMENTER" => Ok(NumpadEnter),
        "NUMPADEQUAL" | "NUMEQUAL" => Ok(NumpadEqual),
        "NUMPADMULTIPLY" | "NUMMULTIPLY" => Ok(NumpadMultiply),
        "NUMPADSUBTRACT" | "NUMSUBTRACT" => Ok(NumpadSubtract),
        "ESCAPE" | "ESC" => Ok(Escape),
        "F1" => Ok(F1),
        "F2" => Ok(F2),
        "F3" => Ok(F3),
        "F4" => Ok(F4),
        "F5" => Ok(F5),
        "F6" => Ok(F6),
        "F7" => Ok(F7),
        "F8" => Ok(F8),
        "F9" => Ok(F9),
        "F10" => Ok(F10),
        "F11" => Ok(F11),
        "F12" => Ok(F12),
        "AUDIOVOLUMEDOWN" | "VOLUMEDOWN" => Ok(AudioVolumeDown),
        "AUDIOVOLUMEUP" | "VOLUMEUP" => Ok(AudioVolumeUp),
        "AUDIOVOLUMEMUTE" | "VOLUMEMUTE" => Ok(AudioVolumeMute),
        "F13" => Ok(F13),
        "F14" => Ok(F14),
        "F15" => Ok(F15),
        "F16" => Ok(F16),
        "F17" => Ok(F17),
        "F18" => Ok(F18),
        "F19" => Ok(F19),
        "F20" => Ok(F20),
        "F21" => Ok(F21),
        "F22" => Ok(F22),
        "F23" => Ok(F23),
        "F24" => Ok(F24),

        _ => Err(Error::UnacceptedKey(key.to_string())),
    }
}
