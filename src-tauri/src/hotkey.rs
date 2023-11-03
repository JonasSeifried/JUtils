use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

pub const MICMUTE: &str = "MicMute";

#[derive(Debug)]
pub struct Hotkey {
    pub name: String,
    pub keys: String,
}

pub fn testing(manager: GlobalHotKeyManager) {
    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    manager.register(hotkey).unwrap();
    let receiver = global_hotkey::GlobalHotKeyEvent::receiver();
    std::thread::spawn(|| loop {
        if let Ok(event) = receiver.try_recv() {
            println!("tray event: {event:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
}
