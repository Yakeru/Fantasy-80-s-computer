use crate::unicode;
use app_macro::*;
use app_macro_derive::AppMacro;

use winit::{
    event::{KeyboardInput, VirtualKeyCode, ElementState},
};

use virtual_frame_buffer::{*, color_palettes::*};
use openweathermap::{Receiver, CurrentWeather};
use std::{time::{Duration, Instant}, f32::consts::PI};

const DEFAULT_BKG_COLOR: u8 = 7;

#[derive(AppMacro)]
pub struct WeatherApp {
    is_shell: bool,
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    receiver: Receiver,
    update_appinterval: Duration,
    last_update: Instant,
    current_weather: Option<Result<CurrentWeather, String>>,
    first_time: bool,
    angle: f32
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
                println!("WeatherApp : Environment variable OWM_KEY not found");
            }
        }

        WeatherApp {
            is_shell: false,
            name: String::from("weather"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            receiver: openweathermap::init("45.4874487,-73.5745913", "metric", "fr", key, 1),
            update_appinterval: Duration::from_secs(5),
            last_update: Instant::now().checked_add(Duration::from_secs(55)).unwrap(),
            current_weather: None,
            first_time: true,
            angle: -PI
        }
    }

    fn update_app(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        let response = AppResponse::new();

        if Instant::now().duration_since(self.last_update) >= self.update_appinterval || self.first_time {
            let last_update = openweathermap::update(&self.receiver);
            if last_update.is_some() {
                self.current_weather = last_update;
            }
            self.last_update = Instant::now();
            self.first_time = false;
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

        
            self.angle += 0.001;

            if self.angle > PI {
                self.angle = -PI
            }
        

        return Some(response);
    }

    fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.clear_frame_buffer(DARK_GREY);
        virtual_frame_buffer.get_console_mut().display = false;

        let x: usize = 100;
        let y: usize = 150;
        let r: usize = 50;

        draw_a_circle(x+1, y-1, r, WHITE, true, virtual_frame_buffer.get_frame_mut());
        draw_a_circle(x-1, y+1, r, BLACK, true, virtual_frame_buffer.get_frame_mut());
        draw_a_circle(x, y, r, YELLOW, true, virtual_frame_buffer.get_frame_mut());
        draw_a_circle(x+1, y-1, r-10, BLACK, true, virtual_frame_buffer.get_frame_mut());
        draw_a_circle(x-1, y+1, r-10, WHITE, true, virtual_frame_buffer.get_frame_mut());
        draw_a_circle(x, y, r-10, LIGHT_GREY, true, virtual_frame_buffer.get_frame_mut());
        draw_a_line(x, y, x, y-r+14, RED, virtual_frame_buffer.get_frame_mut());
        draw_a_line(x+1, y-r+15, x+1, y-r+17, RED, virtual_frame_buffer.get_frame_mut());
        draw_a_line(x-1, y-r+15, x-1, y-r+17, RED, virtual_frame_buffer.get_frame_mut());
        draw_a_circle(x, y, 4, RED, true, virtual_frame_buffer.get_frame_mut());

        draw_a_line_differently(x, y, 30, RED, self.angle, virtual_frame_buffer.get_frame_mut());
        
        match &self.current_weather {
            Some(result) => match result {
                Ok(current_weather) => {

                    virtual_frame_buffer.get_text_layer_mut().insert_string_xy(0, 0, 
                        &format!("Description: {}", current_weather.weather[0].description), Some(WHITE), Some(DARK_GREY), 
                        false, false, false);

                    virtual_frame_buffer.get_text_layer_mut().insert_string_xy(0, 1, 
                        &format!("Temp: {} c", current_weather.main.temp), Some(WHITE), Some(DARK_GREY), 
                        false, false, false);

                    virtual_frame_buffer.get_text_layer_mut().insert_string_xy(0, 2, 
                        &format!("feels like: {} c", current_weather.main.feels_like), Some(WHITE), Some(DARK_GREY), 
                        false, false, false);

                    virtual_frame_buffer.get_text_layer_mut().insert_string_xy(0, 3, 
                        &format!("Humidity: {} %", current_weather.main.humidity), Some(WHITE), Some(DARK_GREY), 
                        false, false, false);

                    virtual_frame_buffer.get_text_layer_mut().insert_string_xy(0, 4, 
                        &format!("Pressure: {} Kpa", current_weather.main.pressure), Some(WHITE), Some(DARK_GREY), 
                        false, false, false);
                }
                Err(message) => {
                    virtual_frame_buffer.get_text_layer_mut().insert_string_xy(0, 0, 
                        message, Some(WHITE), Some(DARK_GREY), 
                        false, false, false);
                }
            },
            None => {
                
            },
        }
    }
}