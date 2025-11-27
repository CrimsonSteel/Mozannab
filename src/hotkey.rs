use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use global_hotkey::{
    GlobalHotKeyManager,
    hotkey::{Code, Modifiers, HotKey},
    GlobalHotKeyEvent,
};

pub struct HotkeyData {
    pub manager: GlobalHotKeyManager,
    pub handle: JoinHandle<()>,
    pub receiver: Receiver<u32>,

    pub toggle_monitoring_id: u32,
    pub calibrate_id: u32,
    pub calibrate_capture_id: u32,
}


pub fn setup_hotkeys() -> Option<HotkeyData> {

    let manager = match GlobalHotKeyManager::new() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to create GlobalHotKeyManager: {:?}", e);
            return None;
        }
    };

    // Register hotkeys

    let toggle_key = HotKey::new(Some(Modifiers::ALT), Code::KeyW);
    let toggle_id = toggle_key.id();
    if let Err(e) = manager.register(toggle_key) {
        eprintln!("Warning: Failed to register Alt+W hotkey: {:?}", e);
        return None;
    }

    let calibrate_key = HotKey::new(Some(Modifiers::ALT), Code::KeyQ);
    let calibrate_id = calibrate_key.id();
    if let Err(e) = manager.register(calibrate_key) {
        eprintln!("Warning: Failed to register Alt+Q hotkey: {:?}", e);
        return None;
    }


    let calibrate_capture_key = HotKey::new(Some(Modifiers::CONTROL), Code::KeyQ);
    let calibrate_capture_id = calibrate_capture_key.id();
    if let Err(e) = manager.register(calibrate_capture_key) {
        eprintln!("Warning: Failed to register Ctrl+Q hotkey: {:?}", e);
        return None;
    }

    let (tx, rx) = mpsc::channel();

    // Listening thread
    let handle = thread::spawn(move || {
        let receiver = GlobalHotKeyEvent::receiver();
        loop {
            if let Ok(event) = receiver.recv() {
                if event.state == global_hotkey::HotKeyState::Pressed {
                    let _ = tx.send(event.id);
                }
            }
        }
    });


    Some(
        HotkeyData {
            manager,
            handle,
            receiver: rx,
            toggle_monitoring_id: toggle_id,
            calibrate_id: calibrate_id,
            calibrate_capture_id: calibrate_capture_id,
        }
    )
}