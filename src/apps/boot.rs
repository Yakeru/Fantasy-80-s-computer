use std::time::Duration;
use app_macro_derive::AppMacro;
use app_macro::AppResponse;
use display_controller::DisplayController;
use winit_input_helper::WinitInputHelper;

use crate::sound::{play::play, notes::*};

#[derive(AppMacro)]
pub struct Boot {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    frame_count: u128,
    starting_time: Duration
}

impl Boot {

    pub fn new() -> Boot {
        Self { enable_auto_escape: true, 
            name: "reboot".to_string(), 
            updating: true, 
            drawing: true, 
            initialized: false,
            frame_count: 0,
            starting_time: Duration::new(0, 0)
         }
    }
    
    pub fn init_app(&mut self, clock: &Clock, dc: &mut DisplayController) {
        dc.get_console_mut().display = false;
        self.frame_count = 0;
        self.starting_time = clock.total_running_time;

        // ************************************************* SOUND TEST **********************************************    
        let mut melody_1: Vec<(Option<f32>, f32)> = Vec::new();
        melody_1.push((None, 1.0));
        melody_1.push((Some(C5), 1.0));
        melody_1.push((None, 1.0));
        melody_1.push((Some(C5), 1.0));
        melody_1.push((Some(F5), 3.0));

        let mut melody_2: Vec<(Option<f32>, f32)> = Vec::new();
        melody_2.push((None, 4.0));
        melody_2.push((Some(A5), 3.0));

        play(480.0, melody_1, melody_2);
    }

    pub fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        clock: &Clock,
        dc: &mut DisplayController,
    ) -> Option<AppResponse> {
        if clock.total_running_time - self.starting_time >= Duration::new(6, 0) {
            self.quit_app(dc);
        }

        if inputs.is_some() && inputs.unwrap().key_pressed(VirtualKeyCode::Escape) {
            self.quit_app(dc);
        }

        None
    }

    pub fn draw_app(
        &mut self,
        clock: &Clock,
        dc: &mut DisplayController,
    ) {
        dc.get_console_mut().display = false;

        //CRT warm up, brightness increases from 0 to 255 and un-distord picture
        let brigthness = if clock.total_running_time - self.starting_time >= Duration::new(2, 0) {
            255
        } else {
            ((clock.total_running_time - self.starting_time).as_millis() * 255 / 2000) as u8
        };

        dc.set_brightness(brigthness);

        //Fill text layer with random garbage
        if self.frame_count == 0 {
            dc.genrate_random_garbage();
        }

        //Clear garbage and display Loading...
        if clock.total_running_time - self.starting_time >= Duration::new(3, 0) {
            dc.get_text_layer_mut().clear();
            dc.clear(0);
            dc.get_text_layer_mut().insert_string_xy(0, 0, "Loading..." , Some(WHITE), Some(BLACK), false, false, false);
        }

        //Display loading overscan while "loading"
        if clock.total_running_time - self.starting_time >= Duration::new(3, 0) && clock.total_running_time - self.starting_time < Duration::new(6, 0) {
            dc.draw_loading_overscan_artefacts();
        }
        self.frame_count += 1;
    }

    fn quit_app(&mut self, dc: &mut DisplayController) {
        self.set_state(false, false);
        self.initialized = false;
        dc.set_brightness(255);
        dc.clear(BLUE);
        dc.get_text_layer_mut().clear();
    }
}