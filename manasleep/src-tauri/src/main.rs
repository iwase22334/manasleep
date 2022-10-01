#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod voice_store;
mod sound_coordinator;
mod ticker;

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
fn cmd_set_volume(volume: u32, tx: tauri::State<mpsc::SyncSender<TickerControl>>) {
    ticker::set_volume(&tx, volume);
}

fn main() {
    let sound_coordinator_tx : mpsc::SyncSender<SoundControl> = sound_coordinator::start();

    let (ticker_control_tx, state_notice_rx)
        : (mpsc::SyncSender<TickerControl>, mpsc::Receiver<TickerStateNotice>)
           = ticker::start(sound_coordinator_tx.clone());

    std::thread::spawn(move || {
        loop {
            match state_notice_rx.recv() {
                Ok(msg) => {
                    match msg {
                        TickerStateNotice::Stopped => { println!("Main: stopped"); },
                        TickerStateNotice::PositionUpdate(n) => { println!("Main: position {:?}", n); },
                    }
                },

                Err(_) => {panic!("Main: Failed to receive");}
            }
        }
    });

    tauri::Builder::default()
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
