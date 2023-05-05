use std::time::Duration;

use crate::Source;

/// An infinite source that produces a sine.
///
/// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct Silence {
    num_sample: usize,
}

impl Silence {
    /// The frequency of the sine.
    #[inline]
    pub fn new() -> Silence {
        Silence {
            num_sample: 0,
        }
    }
}

impl Iterator for Silence {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        Some(0.0)
    }
}

impl Source for Silence {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
