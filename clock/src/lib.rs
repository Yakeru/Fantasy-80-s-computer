use std::time::{Instant, Duration};

#[derive(Clone,Copy)]
pub struct Clock {
    start_time: Instant,
    pub total_running_time: Duration,
    previous_second_tick: Instant,
    previous_half_second_tick: Instant,
    pub second_tick: bool,
    pub second_latch: bool,
    pub half_second_tick: bool,
    pub half_second_latch: bool,
    frame_counter: u128
}

impl Clock {
    pub fn new() -> Clock {
        Clock { 
            start_time: Instant::now(), 
            total_running_time: Duration::new(0,0), 
            previous_second_tick: Instant::now(),
            previous_half_second_tick: Instant::now(), 
            second_tick: false, 
            second_latch: false, 
            half_second_tick: false, 
            half_second_latch: false,
            frame_counter: 0
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.total_running_time = self.start_time.elapsed();

        if self.previous_second_tick.elapsed() >= Duration::new(1, 0) {
            self.previous_second_tick = now;
            self.second_tick = true;
            self.second_latch = !self.second_latch;
        } else {
            self.second_tick = false;
        }

        if self.previous_half_second_tick.elapsed() >= Duration::new(0, 500000000) {
            self.previous_half_second_tick = now;
            self.half_second_tick = true;
            self.half_second_latch = !self.half_second_latch;
        } else {
            self.half_second_tick = false;
        }
    }

    pub fn get_frame_count(&self) -> u128 {
        self.frame_counter
    }

    pub fn count_frame(&mut self) {
        self.frame_counter += 1;
    }
}