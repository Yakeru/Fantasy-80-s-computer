use std::time::Duration;

use rodio::{Source, Sink};

use super::{square::SquareWave};

pub fn play(bpm: f32, track_1: &Vec<Option<(f32, f32)>>, track_2: &Vec<Option<(f32, f32)>>, channel_1: &Sink, channel_2: &Sink) {

    let note_duration: f32 = 60.0 / bpm;

    for note in track_1 {

        match note {
            Some(plop) => {
                channel_1.append(SquareWave::new(plop.0).take_duration(Duration::from_secs_f32(note_duration).mul_f32(plop.1)).amplify(0.05));
            },
            None => {
                channel_1.append(SquareWave::new(0.0).take_duration(Duration::from_secs_f32(note_duration)).amplify(0.05));
            }
        }
    }

    for note in track_2 {

        match note {
            Some(plop) => {
                channel_2.append(SquareWave::new(plop.0).take_duration(Duration::from_secs_f32(note_duration).mul_f32(plop.1)).amplify(0.05));
            },
            None => {
                channel_2.append(SquareWave::new(0.0).take_duration(Duration::from_secs_f32(note_duration)).amplify(0.05));
            }
        }
    }

    channel_1.sleep_until_end();
    channel_2.sleep_until_end();
}