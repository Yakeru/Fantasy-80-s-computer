use app_macro_derive::AppMacro;
use rand::Rng;

use super::{
    config::*,
    theme::{self, *},
};

#[derive(AppMacro)]
pub struct Mandelbrot {
    enable_auto_escape: bool,
    name: String,
    status: AppStatus,
    initialized: bool,
    welcome_screen: bool,
    game: bool,
    menu: bool,
    mandel_x_range: f64,
    mandel_y_range: f64,
    mandel_x_center: f64,
    mandel_y_center: f64,
    max_iteration: usize,
    previous_empty_ratio: f64,
    previous_empty_ratio_delta: f64,
    pause: bool,
    reverse: bool,
    fuzzy: bool,
    themes: Vec<ColorTheme>,
    current_theme: usize,
    palette_rotation: bool,
}

impl Mandelbrot {
    pub fn new() -> Mandelbrot {
        Mandelbrot {
            enable_auto_escape: true,
            name: String::from("mandelbrot"),
            status: AppStatus::Stopped,
            initialized: false,
            welcome_screen: true,
            game: false,
            menu: false,
            mandel_x_center: X_COORD,
            mandel_y_center: Y_COORD,
            mandel_x_range: MAX_X_RANGE,
            mandel_y_range: MAX_Y_RANGE,
            max_iteration: MIN_ITER,
            previous_empty_ratio: 0.0,
            previous_empty_ratio_delta: 0.0,
            pause: true,
            reverse: false,
            fuzzy: true,
            themes: theme::get_themes(),
            current_theme: 2,
            palette_rotation: false,
        }
    }

    pub fn init_app(&mut self, _clock: &Clock, _dc: &mut DisplayController) {
        self.welcome_screen = true;
        self.game = false;
        self.menu = false;
        self.mandel_x_center = X_COORD;
        self.mandel_y_center = Y_COORD;
        self.mandel_x_range = MAX_X_RANGE;
        self.mandel_y_range = MAX_Y_RANGE;
        self.max_iteration = MIN_ITER;
        self.previous_empty_ratio = 0.0;
        self.previous_empty_ratio_delta = 0.0;
        self.pause = true;
        self.reverse = false;
        self.fuzzy = true;
        self.current_theme = 2;
        self.palette_rotation = false;
    }

    pub fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        _clock: &Clock,
    ) -> Option<AppResponse> {
        inputs?;

        let user_inputs = inputs.unwrap();

        /*---------------------------------------------------------- */
        //                 choosing default scenarios
        /*---------------------------------------------------------- */

        if user_inputs.key_pressed(VirtualKeyCode::Space) {
            self.pause = !self.pause;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key1) {
            self.reset();
            self.mandel_x_center = X_COORD;
            self.mandel_y_center = Y_COORD;
            self.current_theme = 2; //tree
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key2) {
            self.reset();
            self.mandel_x_center = -0.749089134879074;
            self.mandel_y_center = 0.04575273713964573;
            self.current_theme = 1; //cool
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key3) {
            self.reset();
            self.mandel_x_center = -1.254716173206939;
            self.mandel_y_center = -0.03269356495238624;
            self.current_theme = 3; //canyon
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key4) {
            self.reset();
            self.mandel_x_center = 0.26781837605081366;
            self.mandel_y_center = -0.003918849643395729;
            self.current_theme = 0; //warm
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key5) {
            self.reset();
            self.mandel_x_center = -0.10971550489778131;
            self.mandel_y_center = 0.8945121343911098;
            self.current_theme = 2; //tree
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key6) {
            self.reset();
            self.mandel_x_center = -1.403277422173161;
            self.mandel_y_center = -0.00022314715329581908;
            self.current_theme = 2; //tree
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key7) {
            self.reset();
            self.mandel_x_center = -0.19827338980477996;
            self.mandel_y_center = -1.100975539162933;
            self.current_theme = 3; //canyon
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key8) {
            self.reset();
            self.mandel_x_center = -1.9425557680573255;
            self.mandel_y_center = 0.0;
            self.current_theme = 1; //cool
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key9) {
            self.reset();
            self.mandel_x_center = 0.35069648168066503;
            self.mandel_y_center = -0.06386659763624122;
            self.current_theme = 4; //?
        }

        /*---------------------------------------------------------- */
        //                      Rendering controls
        /*---------------------------------------------------------- */

        if user_inputs.key_pressed(VirtualKeyCode::Slash) {
            self.reverse = !self.reverse;
        }

        if user_inputs.key_pressed(VirtualKeyCode::R) {
            self.reset();
        }

        if user_inputs.key_pressed(VirtualKeyCode::Comma) {
            self.max_iteration -= 10;
            if self.max_iteration <= MIN_ITER {
                self.max_iteration = MIN_ITER
            };
            println!("max_iteration: {}", self.max_iteration);
        }

        if user_inputs.key_pressed(VirtualKeyCode::Period) {
            self.max_iteration += 10;
            println!("max_iteration: {}", self.max_iteration);
        }

        if user_inputs.key_pressed(VirtualKeyCode::P) {
            self.swap_palette();
        }

        if user_inputs.key_pressed(VirtualKeyCode::F) {
            self.fuzzy = !self.fuzzy;
        }

        if user_inputs.key_pressed(VirtualKeyCode::X) {
            self.palette_rotation = !self.palette_rotation;
        }

        /*---------------------------------------------------------- */
        //                      Movement controls
        /*---------------------------------------------------------- */
        if user_inputs.key_pressed_os(VirtualKeyCode::Up) {
            self.mandel_y_center -= self.mandel_y_range / 50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        }

        if user_inputs.key_pressed_os(VirtualKeyCode::Down) {
            self.mandel_y_center += self.mandel_y_range / 50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        }

        if user_inputs.key_pressed_os(VirtualKeyCode::Left) {
            self.mandel_x_center -= self.mandel_x_range / 50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        }

        if user_inputs.key_pressed_os(VirtualKeyCode::Right) {
            self.mandel_x_center += self.mandel_x_range / 50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        }

        None
    }

    pub fn draw_app(&mut self, clock: &Clock, dc: &mut DisplayController) {
        dc.get_text_layer_mut().clear();
        dc.clear(BLACK);

        if self.palette_rotation && clock.get_frame_count() % 2 == 0 {
            if !self.themes[self.current_theme].get_palette_1().is_empty() {
                self.themes[self.current_theme]
                    .get_palette_1()
                    .rotate_right(1);
            }

            if !self.themes[self.current_theme].get_palette_2().is_empty() {
                self.themes[self.current_theme]
                    .get_palette_2()
                    .rotate_right(1);
            }
        }

        let mandel_x_min: f64 = self.mandel_x_center - self.mandel_x_range / 2.0;
        let mandel_y_min: f64 = self.mandel_y_center - self.mandel_y_range / 2.0;

        let mut max_iteration_count: usize = 0;
        let mut x0: f64;
        let mut y0: f64;
        let mut x: f64;
        let mut y: f64;
        let mut x2: f64;
        let mut y2: f64;
        let mut iteration: usize;
        let mut random = rand::thread_rng();

        // Mandelbrot algorithm from Wikipedia : https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set
        for py in OVERSCAN_V..VIRTUAL_HEIGHT - OVERSCAN_V {
            for px in OVERSCAN_H..VIRTUAL_WIDTH - OVERSCAN_H {
                x0 = ((px as f64 * self.mandel_x_range) / VIRTUAL_WIDTH as f64) + mandel_x_min;
                y0 = ((py as f64 * self.mandel_y_range) / VIRTUAL_HEIGHT as f64) + mandel_y_min;
                x2 = 0.0;
                y2 = 0.0;

                // Add a bit of noise to x0 and y0 to make picture fuzzy
                let fuzziness = self.themes[self.current_theme].fuzzyness;
                if fuzziness > 0.0 && self.fuzzy {
                    x0 += random
                        .gen_range(-self.mandel_x_range / 250.0..self.mandel_x_range / fuzziness);
                    y0 += random
                        .gen_range(-self.mandel_y_range / 250.0..self.mandel_y_range / fuzziness);
                }

                x = 0.0;
                y = 0.0;
                iteration = 0;

                while iteration < self.max_iteration && (x2 + y2) <= 4.0 {
                    x2 = x * x;
                    y2 = y * y;
                    y = (x + x) * y + y0;
                    x = x2 - y2 + x0;
                    iteration += 1;
                }

                // Set pixel color according to iteration nb and palette index
                // if max iteration reached, set to black
                let empty_color = self.themes[self.current_theme].empty_color;
                let color_swap = self.themes[self.current_theme].palette_swap;
                let color_index = iteration % self.themes[self.current_theme].palette1.len();
                let temp_color_1 = self.themes[self.current_theme].palette1[color_index];
                let temp_color_2 = if color_swap {
                    self.themes[self.current_theme].palette2[color_index]
                } else {
                    temp_color_1
                };
                let color: usize = if iteration == self.max_iteration {
                    max_iteration_count += 1;
                    empty_color
                } else if color_swap && random.gen_bool(0.5) {
                    temp_color_2
                } else {
                    temp_color_1
                };

                dc.set_pixel(px as isize, py as isize, color);
            }
        }

        // Increasing the amount of details the deeper we get, to keep the screen filled
        let empty_ratio: f64 = max_iteration_count as f64 / (VIRTUAL_WIDTH * VIRTUAL_HEIGHT) as f64;
        let empty_ratio_delta: f64 = empty_ratio - self.previous_empty_ratio;

        // If the proportion of empty pixels reaches a certain threashold,
        // and the delta between that render and the previous one increases,
        // then increase numer of iterations by 1, to draw more.
        if !self.pause
            && empty_ratio >= self.themes[self.current_theme].empty_ratio
            && empty_ratio_delta >= EMPTY_RATIO_DELTA_TRIGGER
        {
            self.max_iteration += 1;

            // If one more iteration is not enough to reduce the delta, add one more iteration
            if empty_ratio_delta - self.previous_empty_ratio_delta >= EMPTY_RATIO_DELTA_TRIGGER {
                self.max_iteration += 1;
            }
        }

        // If to many pixels are drawn, reduce detail
        // Useful when zooming out, or when going from an empty region to a dense one.
        // different value than EMPTY_RATIO_TRIGGER to avoid +/- EMPTY_RATIO_TRIGGER between two consecutive frames (flickering)
        if !self.pause && self.reverse && empty_ratio < ANTI_EMPTY_RATIO_TRIGGER {
            self.max_iteration -= 1;
            if self.max_iteration < MIN_ITER {
                self.max_iteration = MIN_ITER
            };
            //println!("max_iteration: {}", self.max_iteration);
        }

        // Go deeper, or zoom out, by reducing or increasing the range of values rendered
        // by a fraction of the current range, to maintain constant speed
        if !self.pause {
            if !self.reverse {
                self.mandel_x_range -= self.mandel_x_range / RANGE_DIVIDER_AKA_SPEED;
                self.mandel_y_range -= self.mandel_y_range / RANGE_DIVIDER_AKA_SPEED;
            } else {
                self.mandel_x_range += self.mandel_x_range / RANGE_DIVIDER_AKA_SPEED;
                self.mandel_y_range += self.mandel_y_range / RANGE_DIVIDER_AKA_SPEED;

                if self.mandel_x_range >= MAX_X_RANGE {
                    self.mandel_x_range = MAX_X_RANGE;
                    self.mandel_y_range = MAX_Y_RANGE;
                    self.pause = true;
                    self.reverse = false;
                }
            }
        }

        self.previous_empty_ratio = empty_ratio;
        self.previous_empty_ratio_delta = empty_ratio_delta;

        // If screen gets completely black, reset animation
        if empty_ratio == 1.0 {
            self.reset()
        };

        // If minimum range is reached, reset animation
        if self.mandel_y_range <= MIN_RANGE {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.mandel_x_range = MAX_X_RANGE;
        self.mandel_y_range = MAX_Y_RANGE;
        self.max_iteration = MIN_ITER;
        self.previous_empty_ratio = 0.0;
    }

    fn swap_palette(&mut self) {
        self.current_theme += 1;
        if self.current_theme >= self.themes.len() {
            self.current_theme = 0;
        }
    }
}
