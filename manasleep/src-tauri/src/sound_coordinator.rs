use crate::voice_store;

use rand::prelude::*;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::mpsc;

pub enum SoundControl {
    Play,
    PlayIndex(usize),
    Volume(u32),
    Quit,
}

pub fn start() -> mpsc::SyncSender<SoundControl> {
    let (tx, rx) = mpsc::sync_channel::<SoundControl>(10);

    let voice_store = voice_store::VoiceStore::new();

    std::thread::spawn(move || {
        let (mut _os, mut osh)
            = rodio::OutputStream::try_default().expect("failed to open sound device");

        let sink = Sink::try_new(&osh).expect("failed to create new sink");

        let mut rng = rand::thread_rng();
        let mut indexes: Vec<usize> = (0..voice_store.len()).collect();
        indexes.shuffle(&mut rng);
        let mut iter = indexes.iter();

        loop {
            match rx.recv() {
                Ok(msg) => {
                    match msg {
                        SoundControl::Play => {
                            let mut index = iter.next();
                            if index == None {
                                indexes.shuffle(&mut rng);
                                iter = indexes.iter();
                                index = iter.next();
                            }
                            let index = index.unwrap();

                            println!("sound_coordinator: recv Play {:?}", index);

                            let source = rodio::Decoder::new(
                                std::io::Cursor::new(voice_store.get_data(*index).clone()))
                                    .expect("failed to decord wav");
                            sink.append(source);
                        },

                        SoundControl::PlayIndex(n) => {
                            println!("sound_coordinator: recv PlayIndex {:?}", n);
                            let source = rodio::Decoder::new(
                                std::io::Cursor::new(voice_store.get_data(n).clone()))
                                    .expect("failed to decord wav");
                            sink.append(source);
                        },

                        SoundControl::Volume(n) => {
                            println!("sound_coordinator: recv Volume {:?}", n);
                            sink.set_volume(n as f32 / 100f32);
                        },

                        SoundControl::Quit => {
                            println!("sound_coordinator: recv Quit");
                            break;
                        }
                    }
                },
                Err(err) => { panic!("sound_coordinator: {:?}", err) }
            }

        }

        sink.sleep_until_end();
        println!("sound_coordinator: thread exit");
    });

    return tx;
}
