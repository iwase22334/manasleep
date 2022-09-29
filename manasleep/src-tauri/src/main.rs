#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod voice_store;
mod sound_coordinator;
mod ticker;

use std::fs;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<sound_coordinator::SoundControl>();

    sound_coordinator::start(rx);
    ticker::start(tx);

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
