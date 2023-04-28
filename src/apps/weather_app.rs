use app_macro_derive::AppMacro;
use display_controller::{DisplayController, color_palettes::*};
use openweathermap::{Receiver, CurrentWeather};
use std::{time::{Duration, Instant}, f32::consts::PI};
use chrono::{Timelike, Local};

#[derive(AppMacro)]
pub struct WeatherApp {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    receiver: Receiver,
    update_appinterval: Duration,
    last_update_time: Instant,
    current_weather: Option<Result<CurrentWeather, String>>,
    current_second: u32,
}

impl WeatherApp {
    pub fn new() -> WeatherApp {
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

        // let coord_env: Option<&'static str> = option_env!("COORD");
        // let mut coord = "";

        // match coord_env {
        //     Some(string) => {
        //         coord = string;
        //     },
        //     None => {
        //         println!("WeatherApp : Environment variable COORD not found");
        //     }
        // }

        WeatherApp {
            enable_auto_escape: true,
            name: String::from("weather"),
            updating: false,
            drawing: false,
            initialized: false,
            receiver: openweathermap::init("45.4874487,-73.5745913", "metric", "fr", key, 10),
            update_appinterval: Duration::from_secs(5),
            last_update_time: Instant::now(),
            current_weather: None,
            current_second: 0
        }
    }

    pub fn init_app(&mut self, _clock: &Clock, _dc: &mut DisplayController) {
        openweathermap::update(&self.receiver);
        let now = Local::now();
        self.current_second = now.second();
    }

    fn update_app(
        &mut self, _inputs: Option<&WinitInputHelper>, _clock: &Clock, _dc: &mut DisplayController
    ) -> Option<AppResponse> {
        let response = AppResponse::new();

        if Instant::now().duration_since(self.last_update_time) >= self.update_appinterval {
            let last_weather_update = openweathermap::update(&self.receiver);
            if last_weather_update.is_some() {
                self.current_weather = last_weather_update;
            }
            self.last_update_time = Instant::now();
        }
        
        return Some(response);
    }

    fn draw_app(&mut self, _clock: &Clock, dc: &mut DisplayController) {
        dc.get_text_layer_mut().clear();
        dc.clear(BLACK);
        dc.get_console_mut().display = false;

        // Draw clock analog
        let now = Local::now();
        let hour = now.hour12().1;
        let minute = now.minute();
        let second = now.second();
        let hand_angles = self.time_to_hand_angles(hour, minute, second);
        now.date_naive().format("%d-%m-%Y").to_string();

        let clock_x: usize = VIRTUAL_WIDTH / 2;
        let clock_y: usize = VIRTUAL_HEIGHT / 2 + 32;
        let clock_radius: usize = 50;

        //Clock face
        dc.circle(clock_x+1, clock_y-1, clock_radius, WHITE, WHITE, true);
        dc.circle(clock_x-1, clock_y+1, clock_radius, BLACK, BLACK, true);
        dc.circle(clock_x, clock_y, clock_radius, YELLOW, YELLOW, true);
        dc.circle(clock_x+1, clock_y-1, clock_radius-5, BLACK, BLACK, true);
        dc.circle(clock_x-1, clock_y+1, clock_radius-5, WHITE, WHITE, true);
        dc.circle(clock_x, clock_y, clock_radius-5, LIGHT_GREY, LIGHT_GREY, true);

        let mut index_angle: f32 = 0.0;

        while index_angle < 2.0 * PI {
            let index_position = dc.vector(clock_x, clock_y, 40, LIGHT_GREY, index_angle);
            dc.circle(index_position.0, index_position.1, 1, BLACK, BLACK, true);
            index_angle += PI/6.0;
        }
        
        //Hour hand
        let hour_decoration_coord = dc.vector(clock_x, clock_y, 20, TRUE_BLUE, hand_angles.0);
        dc.vector(hour_decoration_coord.0, hour_decoration_coord.1, 5, TRUE_BLUE, hand_angles.0);
        dc.circle(hour_decoration_coord.0, hour_decoration_coord.1, 2, TRUE_BLUE, TRUE_BLUE, true);
        dc.circle(hour_decoration_coord.0, hour_decoration_coord.1, 1, LIGHT_GREY, LIGHT_GREY, true);
        
        //Minute hand
        let minute_decoration_coord = dc.vector(clock_x, clock_y, 25, TRUE_BLUE, hand_angles.1);
        dc.vector(minute_decoration_coord.0, minute_decoration_coord.1, 7, TRUE_BLUE, hand_angles.1);
        dc.circle(minute_decoration_coord.0, minute_decoration_coord.1, 3, TRUE_BLUE, TRUE_BLUE, true);
        dc.circle(minute_decoration_coord.0, minute_decoration_coord.1, 2, LIGHT_GREY, LIGHT_GREY, true);

        //Second hand
        dc.vector(clock_x, clock_y, 38, DARK_ORANGE, hand_angles.2);

        //center
        dc.circle(clock_x, clock_y, 3, TRUE_BLUE, TRUE_BLUE, true);
        dc.circle(clock_x, clock_y, 2, WHITE, WHITE, true);

        // Draw clock Digital
        if now.hour() < 10 {
            dc.get_text_layer_mut().insert_string_xy(17, 29, 
                &format!("{}", now.hour()), Some(WHITE), Some(BLACK), 
                false, false, false);
        } else {
            dc.get_text_layer_mut().insert_string_xy(16, 29, 
                &format!("{}", now.hour()), Some(WHITE), Some(BLACK), 
                false, false, false);
        }
        dc.get_text_layer_mut().insert_char_xy(18, 29, ':', Some(WHITE), Some(BLACK), 
            false, true, false);
        if now.minute() <10 {
            dc.get_text_layer_mut().insert_string_xy(19, 29, 
                "0", Some(WHITE), Some(BLACK), 
                false, false, false);
            dc.get_text_layer_mut().insert_string_xy(20, 29, 
                &format!("{}", now.minute()), Some(WHITE), Some(BLACK), 
                false, false, false);
        } else {
            dc.get_text_layer_mut().insert_string_xy(19, 29, 
                &format!("{}", now.minute()), Some(WHITE), Some(BLACK), 
                false, false, false);
        }
        
        dc.get_text_layer_mut().insert_char_xy(21, 29, ':', Some(WHITE), Some(BLACK), 
            false, true, false);
        if now.second() < 10 {
            dc.get_text_layer_mut().insert_string_xy(22, 29, 
                "0", Some(WHITE), Some(BLACK), 
                false, false, false);
            dc.get_text_layer_mut().insert_string_xy(23, 29, 
                &format!("{}", now.second()), Some(WHITE), Some(BLACK), 
                false, false, false);
        } else {
            dc.get_text_layer_mut().insert_string_xy(22, 29, 
                &format!("{}", now.second()), Some(WHITE), Some(BLACK), 
                false, false, false);
        }
        
        match &self.current_weather {
            Some(result) => match result {
                Ok(current_weather) => {

                    dc.get_text_layer_mut().insert_string_xy(0, 0, 
                        &format!("Description: {}", current_weather.weather[0].description
                        .replace("é", "e")
                        .replace("è", "e")
                        .replace("à", "a")
                        .replace("ç", "c")), Some(WHITE), Some(BLACK), 
                        false, false, false);

                    dc.get_text_layer_mut().insert_string_xy(0, 2, 
                        &format!("Temperature: {}▪c, feels like {}▪c", current_weather.main.temp, current_weather.main.feels_like), Some(WHITE), Some(BLACK), 
                        false, false, false);

                    dc.get_text_layer_mut().insert_string_xy(0, 4, 
                        &format!("Humidity:    {} %", current_weather.main.humidity), Some(WHITE), Some(BLACK), 
                        false, false, false);

                    dc.get_text_layer_mut().insert_string_xy(0, 6, 
                        &format!("Pressure:    {} Kpa", current_weather.main.pressure), Some(WHITE), Some(BLACK), 
                        false, false, false);
                }
                Err(message) => {
                    dc.get_text_layer_mut().insert_string_xy(0, 0, 
                        message, Some(WHITE), Some(BLACK), 
                        false, false, false);
                }
            },
            None => {
                
            },
        }
    }

    fn time_to_hand_angles(&self, hour: u32, minute: u32, second: u32) -> (f32, f32, f32) {

        // OldRange = (OldMax - OldMin)  
        // NewRange = (NewMax - NewMin)  
        // NewValue = (((OldValue - OldMin) * NewRange) / OldRange) + NewMin
        let hand_range: f32 = 2.0 * PI;
        let hour_range = 12.0;
        let minute_range = 60.0;
        let second_range = 60.0;
    
        let minute_angle_range = 2.0 * PI / 12.0;
        let second_angle_range = 2.0 * PI / 60.0;
        
        let hour_angle = (hour as f32 * hand_range / hour_range - PI/2.0) + (minute as f32 * minute_angle_range / minute_range);
        let minute_angle = (minute as f32 * hand_range / minute_range - PI/2.0) + (second as f32 * second_angle_range / second_range);
        let second_angle = second as f32 * hand_range / second_range - PI/2.0;
    
        (hour_angle, minute_angle, second_angle)
    }
}