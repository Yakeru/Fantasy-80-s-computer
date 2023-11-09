use fantasy_cpc_app_trait::{AppStatus, FantasyCpcApp, FantasyCppAppDefaultParams};
use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::{
    color_palettes::{BLACK, WHITE},
    DisplayController,
};
use std::time::Duration;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::sound::{notes::*, play::play};

pub struct Boot {
    app_params: FantasyCppAppDefaultParams,
    frame_count: u128,
    starting_time: Duration,
}

impl Boot {
    pub fn new() -> Boot {
        Self {
            app_params: FantasyCppAppDefaultParams::new(String::from("reboot"), true),
            frame_count: 0,
            starting_time: Duration::new(0, 0),
        }
    }
}

impl FantasyCpcApp for Boot {
    fn get_app_params(&mut self) -> &mut FantasyCppAppDefaultParams {
        &mut self.app_params
    }

    fn init_app(&mut self, system_clock: &Clock, _display_controller: &mut DisplayController) {
        self.frame_count = 0;
        self.starting_time = system_clock.total_running_time;

        // ************************************************* SOUND TEST **********************************************
        let track_1: Vec<(Option<f32>, f32)> = vec![
            (None, 1.0),
            (Some(C5), 1.0),
            (None, 1.0),
            (Some(C5), 1.0),
            (Some(F5), 3.0),
        ];
        let track_2: Vec<(Option<f32>, f32)> = vec![(None, 4.0), (Some(A5), 3.0)];
        play(480.0, track_1, track_2);
    }

    fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        clock: &fantasy_cpc_clock::Clock,
    ) -> Option<fantasy_cpc_app_trait::AppResponse> {
        if clock.total_running_time - self.starting_time >= Duration::new(6, 0) {
            self.get_app_params().change_status(AppStatus::Stopped);
        }

        if inputs.is_some() && inputs.unwrap().key_pressed(VirtualKeyCode::Escape) {
            self.get_app_params().change_status(AppStatus::Stopped);
        }

        None
    }

    fn draw_app(
        &mut self,
        clock: &fantasy_cpc_clock::Clock,
        display_controller: &mut DisplayController,
    ) {
        //CRT warm up, brightness increases from 0 to 255 and un-distord picture
        let brigthness = if clock.total_running_time - self.starting_time >= Duration::new(2, 0) {
            255
        } else {
            ((clock.total_running_time - self.starting_time).as_millis() * 255 / 2000) as u8
        };

        display_controller.set_brightness(brigthness);

        //Fill text layer with random garbage
        if self.frame_count == 0 {
            display_controller.genrate_random_garbage();
        }

        //Clear garbage and display Loading...
        if clock.total_running_time - self.starting_time >= Duration::new(3, 0) {
            display_controller.get_txt_mut().clear();
            display_controller.clear(0);
            display_controller
                .get_txt_mut()
                .write_str(0, 0, "Loading...");
        }

        //Display loading overscan while "loading"
        if clock.total_running_time - self.starting_time >= Duration::new(3, 0)
            && clock.total_running_time - self.starting_time < Duration::new(6, 0)
        {
            display_controller.draw_loading_overscan_artefacts();
        }
        self.frame_count += 1;
    }
}
