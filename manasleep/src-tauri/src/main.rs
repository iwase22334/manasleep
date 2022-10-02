#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod voice_store;
mod sound_coordinator;
mod ticker;

use tauri::Manager;
use std::sync::mpsc;

use crate::sound_coordinator::{SoundControl};
use crate::ticker::{TickerControl, TickerStateNotice};

#[tauri::command]
fn cmd_set_playing(playing: bool, tx: tauri::State<mpsc::SyncSender<TickerControl>>) {
    ticker::set_playing(&tx, playing);
}

#[tauri::command]
fn cmd_set_looping(looping: bool, tx: tauri::State<mpsc::SyncSender<TickerControl>>) {
    ticker::set_looping(&tx, looping);
}

#[tauri::command]
fn cmd_set_duration(duration: u32, tx: tauri::State<mpsc::SyncSender<TickerControl>>) {
    ticker::set_duration(&tx, duration);
}

#[tauri::command]
fn cmd_set_position(position: u32, tx: tauri::State<mpsc::SyncSender<TickerControl>>) {
    ticker::set_position(&tx, position);
}

#[tauri::command]
fn cmd_set_volume(volume: u32, tx: tauri::State<mpsc::SyncSender<SoundControl>>) {
    tx.send(SoundControl::Volume(volume)).unwrap_or_else(|_| println!("Failed to send volume"));
}

fn start_emitter(rx: mpsc::Receiver<TickerStateNotice>, app_handle: tauri::AppHandle) {
    std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(msg) => {
                match msg {
                    TickerStateNotice::Stopped => {
                        println!("Main: stopped");
                        app_handle
                            .emit_all("player-state-stopped", true)
                            .unwrap();
                    },

                    TickerStateNotice::PositionUpdate(n) => {
                        app_handle
                            .emit_all("player-state-position", n)
                            .unwrap();
                        println!("Main: position {:?}", n);
                    },
                }
            },

            Err(_) => {panic!("Main: Failed to receive");}
        }
    });
}

fn main() {
    let sound_coordinator_tx : mpsc::SyncSender<SoundControl> = sound_coordinator::start();

    let (ticker_control_tx, state_notice_rx)
        : (mpsc::SyncSender<TickerControl>, mpsc::Receiver<TickerStateNotice>)
           = ticker::start(sound_coordinator_tx.clone());

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            start_emitter(state_notice_rx, app_handle);

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())
        })
        .manage(sound_coordinator_tx)
        .manage(ticker_control_tx)
        .invoke_handler(
            tauri::generate_handler![
                cmd_set_playing,
                cmd_set_duration,
                cmd_set_position,
                cmd_set_looping,
                cmd_set_volume,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
