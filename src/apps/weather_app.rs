use chrono::{Local, Timelike};
use fantasy_cpc_app_trait::{AppResponse, AppStatus, FantasyCpcApp, FantasyCppAppDefaultParams};
use fantasy_cpc_display_controller::{
    color_palettes::*,
    config::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH},
    DisplayController,
};
use openweathermap::{CurrentWeather, Receiver};
use rand::Rng;
use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

pub struct WeatherApp {
    app_params: FantasyCppAppDefaultParams,
    receiver: Receiver,
    update_appinterval: Duration,
    last_weather_update: Instant,
    current_weather: Option<Result<CurrentWeather, String>>,
    current_second: u32,
    clouds: Vec<Cloud>,
}

struct Cloud {
    circles: [(isize, isize, usize); 5],
    x: isize,
    y: isize,
}

impl WeatherApp {
    pub fn new() -> WeatherApp {
        let key_env: Option<&'static str> = option_env!("OWM_KEY");
        let mut key = "";

        match key_env {
            Some(string) => {
                key = string;
            }
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
            app_params: FantasyCppAppDefaultParams::new(String::from("weather"), true),
            receiver: openweathermap::init("45.4874487,-73.5745913", "metric", "fr", key, 10),
            update_appinterval: Duration::from_secs(5),
            last_weather_update: Instant::now(),
            current_weather: None,
            current_second: 0,
            clouds: Vec::new(),
        }
    }

    fn draw_analogue_clock(&mut self, dc: &mut DisplayController, clock_x: isize, clock_y: isize) {
        // Draw clock analog
        let now = Local::now();
        let hour = now.hour12().1;
        let minute = now.minute();
        let second = now.second();
        let hand_angles = self.time_to_hand_angles(hour, minute, second);
        now.date_naive().format("%d-%m-%Y").to_string();

        //let clock_x: usize = VIRTUAL_WIDTH / 2;
        //let clock_y: usize = VIRTUAL_HEIGHT / 2 + 32;
        let clock_radius: usize = 50;

        //let render_start = Instant::now();
        dc.clear(BLUE);

        //Square
        //dc.square(clock_x - clock_radius, clock_y - clock_radius, clock_radius * 2, clock_radius * 2, RED, GREEN, true);

        //Clock face
        dc.circle(clock_x + 1, clock_y - 1, clock_radius, WHITE, WHITE, true);
        dc.circle(clock_x - 1, clock_y + 1, clock_radius, BLACK, BLACK, true);
        dc.circle(
            clock_x,
            clock_y,
            clock_radius,
            DARKER_GREY,
            DARKER_GREY,
            true,
        );
        dc.circle(
            clock_x + 1,
            clock_y - 1,
            clock_radius - 5,
            BLACK,
            BLACK,
            true,
        );
        dc.circle(
            clock_x - 1,
            clock_y + 1,
            clock_radius - 5,
            WHITE,
            WHITE,
            true,
        );
        dc.circle(
            clock_x,
            clock_y,
            clock_radius - 5,
            LIGHT_GREY,
            LIGHT_GREY,
            true,
        );

        let mut index_angle: f32 = 0.0;

        while index_angle < 2.0 * PI {
            let index_position = dc.vector(clock_x, clock_y, 40, LIGHT_GREY, index_angle);
            dc.circle(index_position.0, index_position.1, 1, BLACK, WHITE, true);
            index_angle += PI / 6.0;
        }

        //Hour hand
        let hour_decoration_coord = dc.vector(clock_x, clock_y, 20, TRUE_BLUE, hand_angles.0);
        dc.vector(
            hour_decoration_coord.0,
            hour_decoration_coord.1,
            5,
            TRUE_BLUE,
            hand_angles.0,
        );
        dc.circle(
            hour_decoration_coord.0,
            hour_decoration_coord.1,
            2,
            TRUE_BLUE,
            TRUE_BLUE,
            true,
        );
        dc.circle(
            hour_decoration_coord.0,
            hour_decoration_coord.1,
            1,
            LIGHT_GREY,
            LIGHT_GREY,
            true,
        );

        //Minute hand
        let minute_decoration_coord = dc.vector(clock_x, clock_y, 25, TRUE_BLUE, hand_angles.1);
        dc.vector(
            minute_decoration_coord.0,
            minute_decoration_coord.1,
            7,
            TRUE_BLUE,
            hand_angles.1,
        );
        dc.circle(
            minute_decoration_coord.0,
            minute_decoration_coord.1,
            3,
            TRUE_BLUE,
            TRUE_BLUE,
            true,
        );
        dc.circle(
            minute_decoration_coord.0,
            minute_decoration_coord.1,
            2,
            LIGHT_GREY,
            LIGHT_GREY,
            true,
        );

        //Second hand
        dc.vector(clock_x, clock_y, 38, DARK_ORANGE, hand_angles.2);

        //center
        dc.circle(clock_x, clock_y, 3, TRUE_BLUE, TRUE_BLUE, true);
        dc.circle(clock_x, clock_y, 2, WHITE, WHITE, true);
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

        let hour_angle = (hour as f32 * hand_range / hour_range - PI / 2.0)
            + (minute as f32 * minute_angle_range / minute_range);
        let minute_angle = (minute as f32 * hand_range / minute_range - PI / 2.0)
            + (second as f32 * second_angle_range / second_range);
        let second_angle = second as f32 * hand_range / second_range - PI / 2.0;

        (hour_angle, minute_angle, second_angle)
    }

    fn draw_digital_clock(&mut self, dc: &mut DisplayController) {
        let now = Local::now();
        if now.hour() < 10 {
            dc.get_text_layer_mut().insert_string_xy(
                17,
                29,
                &format!("{}", now.hour()),
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
        } else {
            dc.get_text_layer_mut().insert_string_xy(
                16,
                29,
                &format!("{}", now.hour()),
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
        }
        dc.get_text_layer_mut().insert_char_xy(
            18,
            29,
            ':',
            Some(WHITE),
            Some(BLACK),
            false,
            false,
            false,
        );
        if now.minute() < 10 {
            dc.get_text_layer_mut().insert_string_xy(
                19,
                29,
                "0",
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
            dc.get_text_layer_mut().insert_string_xy(
                20,
                29,
                &format!("{}", now.minute()),
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
        } else {
            dc.get_text_layer_mut().insert_string_xy(
                19,
                29,
                &format!("{}", now.minute()),
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
        }

        dc.get_text_layer_mut().insert_char_xy(
            21,
            29,
            ':',
            Some(WHITE),
            Some(BLACK),
            false,
            false,
            false,
        );
        if now.second() < 10 {
            dc.get_text_layer_mut().insert_string_xy(
                22,
                29,
                "0",
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
            dc.get_text_layer_mut().insert_string_xy(
                23,
                29,
                &format!("{}", now.second()),
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
        } else {
            dc.get_text_layer_mut().insert_string_xy(
                22,
                29,
                &format!("{}", now.second()),
                Some(WHITE),
                Some(BLACK),
                false,
                false,
                false,
            );
        }
    }

    fn generate_cloud() -> Cloud {
        //Max box size containing circle centers
        let box_width: isize = 64;
        let box_height: isize = 16;

        //Min and Max cloud circles
        let min_circle_r: usize = 8;
        let max_circle_r: usize = 32;

        //Generate 5 random circles with random radius within box
        let mut random = rand::thread_rng();
        let mut circles: [(isize, isize, usize); 5] = [(0, 0, 0); 5];

        for circle in circles.iter_mut().enumerate() {
            circle.1 .0 = random.gen_range(
                ((box_width / 5) * circle.0 as isize)..((box_width / 5) * (circle.0 as isize + 1)),
            );
            circle.1 .1 = random.gen_range(0..box_height);
            circle.1 .2 = random.gen_range(min_circle_r..=max_circle_r);
        }

        let cloud_x = random.gen_range(-300..-100);
        let cloud_y = random.gen_range(0..80);

        Cloud {
            circles,
            x: cloud_x,
            y: cloud_y,
        }
    }

    fn move_cloud(cloud: &mut Cloud) {
        cloud.x += 1;
    }

    fn draw_cloud(cloud: &mut Cloud, dc: &mut DisplayController) {
        for circle in cloud.circles.iter_mut().enumerate() {
            dc.circle(
                circle.1 .0 + cloud.x - 1,
                circle.1 .1 + cloud.y - 1,
                circle.1 .2,
                LIGHT_GREY,
                LIGHT_GREY,
                true,
            );
            dc.circle(
                circle.1 .0 + cloud.x,
                circle.1 .1 + cloud.y,
                circle.1 .2,
                WHITE,
                WHITE,
                true,
            );
        }
    }
}

impl FantasyCpcApp for WeatherApp {
    fn get_app_params(&mut self) -> &mut FantasyCppAppDefaultParams {
        &mut self.app_params
    }

    fn init_app(
        &mut self,
        system_clock: &fantasy_cpc_clock::Clock,
        display_controller: &mut DisplayController,
    ) {
        openweathermap::update(&self.receiver);

        let now = Local::now();
        self.current_second = now.second();
        self.clouds.clear();
        self.clouds.push(Self::generate_cloud());
        self.clouds.push(Self::generate_cloud());
        self.clouds.push(Self::generate_cloud());
        self.clouds.push(Self::generate_cloud());
    }

    fn update_app(
        &mut self,
        inputs: Option<&winit_input_helper::WinitInputHelper>,
        clock: &fantasy_cpc_clock::Clock,
    ) -> Option<fantasy_cpc_app_trait::AppResponse> {
        let response = AppResponse::new();

        if Instant::now().duration_since(self.last_weather_update) >= self.update_appinterval {
            let last_weather_update = openweathermap::update(&self.receiver);
            if last_weather_update.is_some() {
                self.current_weather = last_weather_update;
            }
            self.last_weather_update = Instant::now();
        }

        if clock.get_frame_count() % 10 == 0 {
            let mut clouds_to_pop: Vec<usize> = Vec::new();
            for (index, cloud) in self.clouds.chunks_exact_mut(1).enumerate() {
                Self::move_cloud(&mut cloud[0]);
                if cloud[0].x > VIRTUAL_WIDTH as isize + 100 {
                    clouds_to_pop.push(index);
                }
            }
            for index in clouds_to_pop {
                self.clouds.remove(index);
                self.clouds.push(Self::generate_cloud());
            }
        }

        None
    }

    fn draw_app(
        &mut self,
        clock: &fantasy_cpc_clock::Clock,
        display_controller: &mut DisplayController,
    ) {
        display_controller.get_text_layer_mut().clear();
        display_controller.clear(BLACK);

        self.draw_analogue_clock(
            display_controller,
            (VIRTUAL_WIDTH / 2) as isize,
            (VIRTUAL_HEIGHT - 75) as isize,
        );
        self.draw_digital_clock(display_controller);

        match &self.current_weather {
            Some(result) => match result {
                Ok(current_weather) => {
                    display_controller.get_text_layer_mut().insert_string_xy(
                        0,
                        0,
                        &format!(
                            "Description: {}",
                            current_weather.weather[0]
                                .description
                                .replace(['é', 'ê', 'è'], "e")
                                .replace('à', "a")
                                .replace('ç', "c")
                        ),
                        Some(WHITE),
                        Some(BLACK),
                        false,
                        false,
                        false,
                    );

                    display_controller.get_text_layer_mut().insert_string_xy(
                        0,
                        2,
                        &format!(
                            "Temperature: {}▪c, feels like {}▪c",
                            current_weather.main.temp, current_weather.main.feels_like
                        ),
                        Some(WHITE),
                        Some(BLACK),
                        false,
                        false,
                        false,
                    );

                    display_controller.get_text_layer_mut().insert_string_xy(
                        0,
                        4,
                        &format!("Humidity:    {} %", current_weather.main.humidity),
                        Some(WHITE),
                        Some(BLACK),
                        false,
                        false,
                        false,
                    );

                    display_controller.get_text_layer_mut().insert_string_xy(
                        0,
                        6,
                        &format!("Pressure:    {} Kpa", current_weather.main.pressure),
                        Some(WHITE),
                        Some(BLACK),
                        false,
                        false,
                        false,
                    );

                    display_controller.get_text_layer_mut().insert_string_xy(
                        0,
                        8,
                        &format!("Wind:        {} m/s", current_weather.wind.speed),
                        Some(WHITE),
                        Some(BLACK),
                        false,
                        false,
                        false,
                    );
                }
                Err(message) => {
                    display_controller.get_text_layer_mut().insert_string_xy(
                        0,
                        0,
                        message,
                        Some(WHITE),
                        Some(BLACK),
                        false,
                        false,
                        false,
                    );
                }
            },
            None => (),
        }

        for cloud in self.clouds.chunks_exact_mut(1) {
            Self::draw_cloud(&mut cloud[0], display_controller);
        }
    }
}
