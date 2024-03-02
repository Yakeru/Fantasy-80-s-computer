use fantasy_cpc_app_trait::{AppMessage, AppStatus, FantasyCpcApp, FantasyCppAppDefaultParams};
use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::{
    characters_rom,
    color_palettes::PALETE_SIZE,
    config::{TEXT_COLUMNS, TEXT_ROWS, VIRTUAL_HEIGHT},
    text_layer::{Text, TextCell, TextCellStyle, DEFAULT_STYLE},
    DisplayController,
};
use rand::Rng;
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

    fn generate_random_garbage(&mut self, dc: &mut DisplayController) {
        let mut random = rand::thread_rng();

        let rnd_clear_color: usize = random.gen_range(0..32);
        dc.clear(rnd_clear_color, None);

        for y in 0..TEXT_ROWS {
            for x in 0..TEXT_COLUMNS {
                let mut color: usize = random.gen_range(0..(PALETE_SIZE + 10)); //To get a bit more black
                color = if color > PALETE_SIZE - 1 { 0 } else { color };

                let mut bkg_color: usize = random.gen_range(0..(PALETE_SIZE + 10));
                bkg_color = if bkg_color > PALETE_SIZE - 1 {
                    0
                } else {
                    bkg_color
                };

                let mut random_char_index = random.gen_range(0..100);
                random_char_index = if random_char_index > characters_rom::CHAR_TABLE.len() - 1 {
                    0
                } else {
                    random_char_index
                };
                let c: char = characters_rom::CHAR_TABLE[random_char_index];
                let effect: u8 = random.gen_range(0..32);
                let swap_color: bool = effect & 0b00000001 > 0;
                let blink: bool = effect & 0b00000010 > 0;
                let shadowed: bool = effect & 0b00000100 > 0;
                let flip_h: bool = effect & 0b00001000 > 0;
                let flip_v: bool = effect & 0b00010000 > 0;

                let text_cell: TextCell = TextCell {
                    c: Some(c),
                    style: Some(TextCellStyle {
                        color,
                        bkg_color,
                        swap_color,
                        blink,
                        shadowed,
                        flip_h,
                        flip_v,
                    }),
                };

                dc.text_layer.map[y][x] = text_cell;
            }
        }
    }

    fn draw_loading_overscan_artefacts(&mut self, dc: &mut DisplayController) {
        let mut random = rand::thread_rng();
        let mut rgb_color: usize = random.gen_range(0..32);
        let mut line_count: usize = 0;
        let mut band_height: usize = random.gen_range(4..20);

        while line_count <= VIRTUAL_HEIGHT {
            let range_max = if line_count + band_height > VIRTUAL_HEIGHT {
                VIRTUAL_HEIGHT
            } else {
                line_count + band_height
            };
            dc.set_overscan_color_range(rgb_color, line_count..range_max);
            line_count += band_height;
            rgb_color = random.gen_range(0..32);
            band_height = random.gen_range(4..20);
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
        _messages: Option<Vec<AppMessage>>,
        clock: &fantasy_cpc_clock::Clock,
    ) -> Option<Vec<AppMessage>> {
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

        display_controller.brightness = brigthness;

        //Fill text layer with random garbage
        if self.frame_count == 0 {
            self.generate_random_garbage(display_controller);
        }

        //Clear garbage and display Loading...
        if clock.total_running_time - self.starting_time >= Duration::new(3, 0) {
            display_controller.clear(0, None);
            display_controller.write(0, 0, Text::String("Loading..."));
        }

        //Display loading overscan while "loading"
        if clock.total_running_time - self.starting_time >= Duration::new(3, 0)
            && clock.total_running_time - self.starting_time < Duration::new(6, 0)
        {
            self.draw_loading_overscan_artefacts(display_controller);
        }
        self.frame_count += 1;
    }
}
