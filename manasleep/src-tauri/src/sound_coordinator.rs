use crate::voice_store;

use rand::prelude::*;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::mpsc;

pub enum SoundControl {
    Play,
    PlayIndex(usize),
    Quit,
}

pub fn start() -> mpsc::SyncSender<SoundControl> {
    let (tx, rx) = mpsc::sync_channel::<SoundControl>(1);

    let voice_store = voice_store::VoiceStore::new();

    let mut rng = rand::thread_rng();
    let mut indexex: Vec<usize> = (0..voice_store.len()).collect();
    indexex.shuffle(&mut rng);

    std::thread::spawn(move || {
        let (mut _os, mut osh)
            = rodio::OutputStream::try_default().expect("failed to open sound device");

        let sink = Sink::try_new(&osh).expect("failed to create new sink");
        for index in indexex {
            match rx.recv() {
                Ok(msg) => {
                    match msg {
                        SoundControl::Play => {},
                        SoundControl::PlayIndex(n) => {},
                        SoundControl::Quit => { break; }
                    }
                },
                Err(_) => { panic!("disconnected") }
            }

            let source = rodio::Decoder::new(
                std::io::Cursor::new(voice_store.get_data(index).clone())).expect("failed to decord wav");

            sink.append(source);
        }

        sink.sleep_until_end();
        println!("thread exit");
    });

    return tx;
}
