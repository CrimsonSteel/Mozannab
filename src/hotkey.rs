use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;


// For storing them in a json config
#[derive(Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub toggle_monitoring: String, 
    pub calibrate: String,         
    pub calibrate_capture: String, 
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            toggle_monitoring: "Alt+W".to_string(),
            calibrate: "Alt+Q".to_string(),
            calibrate_capture: "Ctrl+Q".to_string(),
        }
    }
}

impl HotkeyConfig {
    pub fn load() -> Self {
        let appdata = std::env::var("APPDATA")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        
        let config_dir = PathBuf::from(appdata).join("Mozannab");
        let _ = fs::create_dir_all(&config_dir);
        let config_path = config_dir.join("hotkeys.json");
        
        if let Ok(content) = fs::read_to_string(&config_path) {
            serde_json::from_str(&content).unwrap_or_else(|_| {
                let default = Self::default();
                let _ = fs::write(&config_path, serde_json::to_string_pretty(&default).unwrap());
                default
            })
        } else {
            let default = Self::default();
            let _ = fs::write(&config_path, serde_json::to_string_pretty(&default).unwrap());
            default
        }
    }
}

fn parse_hotkey(s: &str) -> Option<(Option<Modifiers>, Code)> {
    let parts: Vec<&str> = s.split('+').collect();
    if parts.is_empty() {
        return None;
    }
    
    let mut modifiers = Modifiers::empty();
    let key_str = parts.last()?;
    
    for part in &parts[..parts.len() - 1] {
        match part.trim().to_lowercase().as_str() {
            "alt" => modifiers |= Modifiers::ALT,
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "super" | "win" | "cmd" => modifiers |= Modifiers::SUPER,
            _ => {}
        }
    }
    
    let code = match key_str.trim().to_uppercase().as_str() {
        "Q" => Code::KeyQ,
        "W" => Code::KeyW,
        "E" => Code::KeyE,
        "R" => Code::KeyR,
        "T" => Code::KeyT,
        "Y" => Code::KeyY,
        "U" => Code::KeyU,
        "I" => Code::KeyI,
        "O" => Code::KeyO,
        "P" => Code::KeyP,
        "A" => Code::KeyA,
        "S" => Code::KeyS,
        "D" => Code::KeyD,
        "F" => Code::KeyF,
        "G" => Code::KeyG,
        "H" => Code::KeyH,
        "J" => Code::KeyJ,
        "K" => Code::KeyK,
        "L" => Code::KeyL,
        "Z" => Code::KeyZ,
        "X" => Code::KeyX,
        "C" => Code::KeyC,
        "V" => Code::KeyV,
        "B" => Code::KeyB,
        "N" => Code::KeyN,
        "M" => Code::KeyM,
        _ => return None,
    };
    
    let mods = if modifiers.is_empty() { None } else { Some(modifiers) };
    Some((mods, code))
}

pub struct HotkeyData {
    pub manager: GlobalHotKeyManager,
    pub handle: JoinHandle<()>,
    pub receiver: Receiver<u32>,

    pub toggle_monitoring_id: u32,
    pub calibrate_id: u32,
    pub calibrate_capture_id: u32,

    // Needed as I need to show it in a popup
    pub calibrate_capture_str: String,
}

pub fn setup_hotkeys() -> Option<HotkeyData> {

    let config = HotkeyConfig::load();

    let manager = match GlobalHotKeyManager::new() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to create GlobalHotKeyManager: {e:?}");
            return None;
        }
    };

    // Register hotkeys

    let (toggle_mods, toggle_code) = parse_hotkey(&config.toggle_monitoring)?;
    let toggle_key = HotKey::new(toggle_mods, toggle_code);
    let toggle_id = toggle_key.id();
    if let Err(e) = manager.register(toggle_key) {
        eprintln!("Warning: Failed to register Alt+W hotkey: {e:?}");
        return None;
    }

    let (cal_mods, cal_code) = parse_hotkey(&config.calibrate)?;
    let calibrate_key = HotKey::new(cal_mods, cal_code);
    let calibrate_id = calibrate_key.id();
    if let Err(e) = manager.register(calibrate_key) {
        eprintln!("Warning: Failed to register Alt+Q hotkey: {e:?}");
        return None;
    }

    
    let (cal_cap_mods, cal_cap_code) = parse_hotkey(&config.calibrate_capture)?;
    let calibrate_capture_key = HotKey::new(cal_cap_mods, cal_cap_code);
    let calibrate_capture_id = calibrate_capture_key.id();
    if let Err(e) = manager.register(calibrate_capture_key) {
        eprintln!("Warning: Failed to register Ctrl+Q hotkey: {e:?}");
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

    Some(HotkeyData {
        manager,
        handle,
        receiver: rx,
        toggle_monitoring_id: toggle_id,
        calibrate_id,
        calibrate_capture_id,
        calibrate_capture_str: config.calibrate_capture,
    })
}
