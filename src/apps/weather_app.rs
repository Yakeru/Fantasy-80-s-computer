use crate::unicode;
use app_macro::*;
use app_macro_derive::AppMacro;

// use crate::text_layer::TextLayerChar;
use winit::event::{VirtualKeyCode, KeyboardInput};

use virtual_frame_buffer::*;
use openweathermap::Receiver;
use std::time::{Duration, Instant};

const DEFAULT_BKG_COLOR: u8 = 7;
const DEFAULT_COLOR: u8 = 0;

#[derive(AppMacro)]
pub struct WeatherApp {
    name: String,
    selected_color: u8,
    selected_bkg_color: u8,
    columns: u8,
    rows: u8,
    // buffer: Vec<TextLayerChar>,
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

        let key_env: Option<&'static str> = option_env!("SECRET_KEY");
        let mut key = ""; 

        match key_env {
            Some(string) => {
                key = string;
            },

            None => ()
        }

        WeatherApp {
            name: String::from("Weather"),
            selected_color: DEFAULT_COLOR,
            selected_bkg_color: DEFAULT_BKG_COLOR,
            columns: 0,
            rows: 0,
            // buffer,
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
        char_received: Option<char>
    ) -> AppResponse {
        let mut response = AppResponse::new();

        //if Instant::now().duration_since(self.last_update) >= self.update_appinterval {
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
        //}

        if !self.started {
            self.start();
            self.started = true;
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

        return response;
    }

    fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        // virtual_frame_buffer.get_text_layer().clear();
        // virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
        // virtual_frame_buffer
        //     .get_text_layer()
        //     .push_string(&self.message, None, None, false);
    }
}
