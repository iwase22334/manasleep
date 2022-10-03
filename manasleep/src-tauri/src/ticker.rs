use std::time::{Duration};
use crate::sound_coordinator::{SoundControl};
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError::{Timeout, Disconnected};

pub enum TickerControl {
    Playing(bool),
    Looping(bool),
    Duration(u32),
    Interval(u32),
    Position(u32),
}

pub enum TickerStateNotice {
    Stopped,
    PositionUpdate(u32),
}

pub fn start(sound_control: mpsc::SyncSender<SoundControl>)
    -> (mpsc::SyncSender<TickerControl>, mpsc::Receiver<TickerStateNotice>) {

    let (ticker_control_tx, ticker_control_rx)
        = mpsc::sync_channel::<TickerControl>(1);

    let (state_notice_tx, state_notice_rx)
        = mpsc::sync_channel::<TickerStateNotice>(1);

    let tick = Duration::from_millis(1000);
    let mut active_duration = Duration::from_millis(1000 * 60 * 30);
    let mut interval = Duration::from_millis(10000);
    let mut position = Duration::from_millis(0);
    let mut play_position = Duration::from_millis(0);

    let mut playing: bool = false;
    let mut looping: bool = false;

    std::thread::spawn(move || {
        loop {
            match ticker_control_rx.recv_timeout(tick) {
                Ok(msg) => {
                    match msg {
                        TickerControl::Playing(b) => {
                            println!("recv Playing {:?}", b);
                            playing = b;
                            continue;
                        },

                        TickerControl::Looping(b) => {
                            println!("recv Looping {:?}", b);
                            looping = b;
                            continue;
                        },

                        TickerControl::Duration(n) => {
                            println!("recv Duration {:?}", n);
                            active_duration = Duration::from_secs(n.into());
                            continue;
                        },

                        TickerControl::Interval(n) => {
                            println!("recv Interval {:?}", n);
                            interval = Duration::from_secs(n.into());
                            continue;
                        },

                        TickerControl::Position(n) => {
                            println!("recv Position {:?}", n);
                            position = Duration::from_secs(n.into());
                            play_position = Duration::from_secs(0);
                            continue;
                        },
                    }
                },
                Err(err) if err == Timeout => {
                },
                Err(err) if err == Disconnected => { panic!("disconnected") }
                Err(_) => { panic!("Unknown error") }
            }

            if playing {
                position += tick;

                if position > active_duration {
                    if !looping {
                        playing = false;
                        state_notice_tx.try_send(TickerStateNotice::Stopped)
                            .unwrap_or_else(|_| {println!("Failed to try_send Stopped")});
                    } else {
                        position = Duration::from_millis(0);
                    }
                }

                play_position += tick;

                state_notice_tx.try_send(
                    TickerStateNotice::PositionUpdate(
                        position.as_secs().try_into().unwrap()))
                            .unwrap_or_else(|_| {println!("Failed to try_send PositionUpdate")});

                if play_position >= interval {
                    sound_control.send(SoundControl::Play).expect("Failed to send play");
                    play_position = Duration::from_millis(0);
                }
            }

            print!("ticker:");
            print!(" active_duration {:?}", active_duration);
            print!(" interval {:?}", interval);
            print!(" tick {:?}", tick);
            print!(" position {:?}", position);
            print!(" play_position {:?}", play_position);
            print!(" playing {:?}", playing);
            println!(" looping {:?}", looping);
        }

        sound_control.send(SoundControl::Quit).expect("Failed to send quit");
    });

    return (ticker_control_tx, state_notice_rx);
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

pub fn set_interval(tx: &std::sync::mpsc::SyncSender<TickerControl>, interval: u32) {
    tx.send(TickerControl::Interval(interval)).unwrap();
}

pub fn set_position(tx: &std::sync::mpsc::SyncSender<TickerControl>, position: u32) {
    tx.send(TickerControl::Position(position)).unwrap();
}
