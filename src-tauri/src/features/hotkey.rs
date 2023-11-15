use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::error::{Error, Result};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
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

    pub fn register_hotkey(&self, name: &str, keys: &str) -> Result<()> {
        self.unregister_hotkey(name)?;
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
        self.required_hotkeys
            .lock()
            .unwrap()
            .insert(name.into(), hotkeys.iter().map(|hk| hk.id()).collect());
        Ok(())
    }

    fn unregister_hotkey(&self, name: &str) -> Result<()> {
        if let Some(hotkey_ids) = self.required_hotkeys.lock().unwrap().get(name) {
            let mut hotkeys: Vec<HotKey> = Vec::new();
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
            self.manager.unregister_all(&hotkeys)?;
            self.required_hotkeys
                .lock()
                .unwrap()
                .remove(name)
                .expect("registed hotkeys should be containted in  required_hotkeys");
        }
        Ok(())
    }
}

fn parse_hotkey(keys: &str) -> Result<Vec<HotKey>> {
    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    let hotkey2 = HotKey::new(Some(Modifiers::SHIFT), Code::KeyF);
    Ok(vec![hotkey, hotkey2])
}

#[derive(Debug)]
pub struct RegisteredHotkey {
    pub name: String,
    pub hotkey: HotKey,
}

#[derive(Debug)]
pub struct Hotkey {
    pub name: String,
    pub keys: String,
}

pub fn testing(hotkey_manager: &HotKeyManager, app: &mut tauri::App) {
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
