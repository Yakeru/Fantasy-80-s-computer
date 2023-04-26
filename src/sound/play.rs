use std::{time::Duration, thread};
use rodio::{Source, Sink, OutputStream};
use super::{square::SquareWave, silence::Silence};

/// First time sound is played, it takes a few seconds and gets de-sync'ed with the app
/// So here is a function to play an empty sound for 1/10 s to "init" rodio
pub fn init_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(Silence::new().take_duration(Duration::from_secs_f32(0.1)));
    sink.sleep_until_end();
}

pub fn play(bpm: f32, track_1: Vec<(Option<f32>, f32)>, track_2: Vec<(Option<f32>, f32)>) {

    let handle = thread::Builder::new().name("sound".to_string()).spawn(move || {

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let channel_1 = Sink::try_new(&stream_handle).unwrap();
        let channel_2 = Sink::try_new(&stream_handle).unwrap();
        channel_1.set_volume(1.0);
        channel_2.set_volume(1.0);

        let note_duration: f32 = 60.0 / bpm;

        for note in track_1 {

            match note.0 {
                Some(plop) => {
                    channel_1.append(SquareWave::new(plop).take_duration(Duration::from_secs_f32(note_duration).mul_f32(note.1)));
                },
                None => {
                    channel_1.append(Silence::new().take_duration(Duration::from_secs_f32(note_duration).mul_f32(note.1)));
                }
            }
        }

        for note in track_2 {

            match note.0 {
                Some(plop) => {
                    channel_2.append(SquareWave::new(plop).take_duration(Duration::from_secs_f32(note_duration).mul_f32(note.1)));
                },
                None => {
                    channel_2.append(Silence::new().take_duration(Duration::from_secs_f32(note_duration).mul_f32(note.1)));
                }
            }
        }

        channel_1.sleep_until_end();
        channel_2.sleep_until_end();
    });
    
    match handle {
    Ok(v) => println!("working with version: {v:?}"),
    Err(e) => println!("error parsing header: {e:?}"),
}

}