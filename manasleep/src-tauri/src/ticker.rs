use std::time::{Duration};
use crate::sound_coordinator::{SoundControl};

pub fn start(tx: std::sync::mpsc::Sender<SoundControl>) {
    let active_duration = Duration::from_millis(1000 * 60 * 30);
    let tick_duration = Duration::from_millis(10000);
    let mut spend = Duration::from_millis(0);

    std::thread::spawn(move || {
        while spend + tick_duration <= active_duration {
            std::thread::sleep(tick_duration);
            tx.send(SoundControl::Play).expect("Failed to send");
            spend += tick_duration;
        }

        tx.send(SoundControl::Quit).expect("Failed to send");
    });

}
