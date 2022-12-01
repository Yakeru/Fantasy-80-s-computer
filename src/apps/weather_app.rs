use crate::unicode;
use app_macro::*;
use app_macro_derive::AppMacro;

// use crate::text_layer::TextLayerChar;
use winit::event::{KeyboardInput};

use virtual_frame_buffer::{*, color_palettes::{DARKGREY, WHITE}};
use openweathermap::Receiver;
use std::time::{Duration, Instant};

const DEFAULT_BKG_COLOR: u8 = 7;

#[derive(AppMacro)]
pub struct WeatherApp {
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    receiver: Receiver,
    update_appinterval: Duration,
    last_update: Instant,
    message: String,
}

impl WeatherApp {
    pub fn new() -> WeatherApp {
        // let buffer = Vec::new();

        let key_env: Option<&'static str> = option_env!("OWM_KEY");
        let mut key = ""; 

        match key_env {
            Some(string) => {
                key = string;
            },

            None => {
                println!("WeatherApp : Environment variable not found");
            }
        }

        WeatherApp {
            name: String::from("Weather"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            receiver: openweathermap::init("45.4874487,-73.5745913", "metric", "fr", key, 1),
            update_appinterval: Duration::from_secs(60),
            last_update: Instant::now().checked_add(Duration::from_secs(55)).unwrap(),
            message: String::from("Loading..."),
        }
    }

    fn update_app(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        let response = AppResponse::new();

        if Instant::now().duration_since(self.last_update) >= self.update_appinterval {
            let weather = openweathermap::update(&self.receiver);

            match weather {
                Some(result) => match result {
                    Ok(current_weather) => {
                        self.message = format!("Temp: {}c\u{000D}feels like: {}c\u{000D}Humidity: {}%\u{000D}Pressure: {}Kpa\u{000D}Description: {}", current_weather.main.temp, 
                                current_weather.main.feels_like, current_weather.main.humidity, current_weather.main.pressure, &current_weather.weather[0].description);
                    }
                    Err(message) => println!("OpenWeather API message {}", message),
                },
                None => (),
            }

            self.last_update = Instant::now();
        }

        if !self.started {
            self.start();
            self.started = true;
        }

        match keybord_input {
            Some(_c) => (),
            None => ()
        }

        match char_received {
            Some(c) => {
                match c {
                    unicode::BACKSPACE => {
                    }

                    unicode::ENTER => {
                    }

                    unicode::ESCAPE => {
                        self.updating = false;
                        self.drawing = false;
                        self.end();
                    }

                    _ => {
                        // let plop: TextLayerChar = TextLayerChar {
                        //     unicode: c,
                        //     color: self.selected_color,
                        //     background_color: self.selected_bkg_color,
                        //     blink: false,
                        //     flipp: false
                        // };

                        // self.buffer.push(plop);
                    }
                }
            }
            None => (),
        }

        return Some(response);
    }

    fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.clear_frame_buffer(DARKGREY);
        virtual_frame_buffer
            .get_text_layer_mut().insert_string_xy(0, 0, &self.message, Some(WHITE), Some(DARKGREY), false, false, false);
    }
}
