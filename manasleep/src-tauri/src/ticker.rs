use std::time::{Duration};
use crate::sound_coordinator::{SoundControl};
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError::{Timeout, Disconnected};

pub enum TickerControl {
    Playing(bool),
    Looping(bool),
    Duration(u32),
    Position(u32),
    Volume(u32),
}

pub fn start(sound_control: mpsc::SyncSender<SoundControl>) -> mpsc::SyncSender<TickerControl>{
    let (ticker_control_tx, ticker_control_rx)
        = mpsc::sync_channel::<TickerControl>(1);

    let active_duration = Duration::from_millis(1000 * 60 * 30);
    let tick_duration = Duration::from_millis(10000);
    let mut spend = Duration::from_millis(0);

    std::thread::spawn(move || {
        while spend + tick_duration <= active_duration {
            match ticker_control_rx.recv_timeout(tick_duration) {
                Ok(msg) => {
                    match msg {
                        TickerControl::Playing(b) => { println!("recv Playing {:?}", b) },
                        TickerControl::Looping(b) => { println!("recv Looping {:?}", b) },
                        TickerControl::Duration(n) => { println!("recv Duration {:?}", n) },
                        TickerControl::Position(n) => { println!("recv Position {:?}", n) },
                        TickerControl::Volume(n) => { println!("recv Volume {:?}", n) },
                    }
                },
                Err(err) if err == Timeout => {},
                Err(err) if err == Disconnected => { panic!("disconnected") }
                Err(_) => { panic!("Unknown error") }
            }

            sound_control.send(SoundControl::Play).expect("Failed to send");
            spend += tick_duration;
        }

        sound_control.send(SoundControl::Quit).expect("Failed to send");
    });

    return ticker_control_tx;
}

pub fn set_playing(tx: &std::sync::mpsc::SyncSender<TickerControl>, playing: bool) {
    tx.send(TickerControl::Playing(playing)).unwrap();
}

pub fn set_looping(tx: &std::sync::mpsc::SyncSender<TickerControl>, looping: bool) {
    tx.send(TickerControl::Looping(looping)).unwrap();
}

pub fn set_duration(tx: &std::sync::mpsc::SyncSender<TickerControl>, duration: u32) {
    tx.send(TickerControl::Duration(duration)).unwrap();
}

pub fn set_position(tx: &std::sync::mpsc::SyncSender<TickerControl>, position: u32) {
    tx.send(TickerControl::Position(position)).unwrap();
}

pub fn set_volume(tx: &std::sync::mpsc::SyncSender<TickerControl>, volume: u32) {
    tx.send(TickerControl::Volume(volume)).unwrap();
}
