use app_macro_derive::AppMacro;

const MIN_ITER: usize = 100;
const MAX_X_RANGE: f64 = 2.47;
const MAX_Y_RANGE: f64 = 1.8976471;
const MIN_RANGE: f64 = 0.0000000000000007;
const X_COORD: f64 = -1.0126192432058039;
const Y_COORD: f64 = -0.32202936226897944;
const EMPTY_RATIO_TRIGGER: f64 = 0.1;
const EMPTY_RATIO_DELTA_TRIGGER: f64 = 0.0025;
const RANGE_DIVIDER_AKA_SPEED: f64 = 70.0;

const WARM_PALETTE: [u8;10] = [BROWNISH_BLACK, DARK_BROWN, BROWN, DARK_RED, RED, DARK_ORANGE, ORANGE, YELLOW, LIGHT_PEACH, WHITE];
const COOL_PALETTE: [u8;9] = [DARK_PURPLE, DARKER_PURPLE, DARKER_BLUE, DARK_BLUE, TRUE_BLUE, BLUE, WHITE, LAVENDER, MAUVE];
const TREE_PALETTE: [u8;14] = [DARK_BROWN, BROWN, DARK_BROWN, BROWN, DARK_BROWN, BROWN, DARK_BROWN, BROWN, DARK_BROWN, BROWN, DARK_GREEN, MEDIUM_GREEN, GREEN, LIME_GREEN];


#[derive(AppMacro)]
pub struct Mandelbrot {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
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
}

impl Mandelbrot {
    pub fn new() -> Mandelbrot {
        Mandelbrot {
            enable_auto_escape: false,
            name: String::from("mandelbrot"),
            updating: false,
            drawing: false,
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
        }
    }

    pub fn init_app(&mut self, _virtual_frame_buffer: &mut VirtualFrameBuffer) {
        self.welcome_screen = true;
        self.game = false;
        self.menu = false;
    }

    pub fn update_app(
        &mut self,
        inputs: &WinitInputHelper,
        _clock: &Clock,
        virtual_frame_buffer: &mut VirtualFrameBuffer,
    ) -> Option<AppResponse> {
        // if self.welcome_screen {
        //     self.update_welcome_screen(inputs, virtual_frame_buffer);
        // } else if self.game {
        //     self.update_game(inputs, virtual_frame_buffer);
        // } else {
        //     self.update_menu(inputs, virtual_frame_buffer);
        // }
        self.update_welcome_screen(inputs, virtual_frame_buffer);
        return None;
    }

    pub fn draw_app(
        &mut self,
        inputs: &WinitInputHelper,
        clock: &Clock,
        virtual_frame_buffer: &mut VirtualFrameBuffer,
    ) {
        // if self.welcome_screen {
        //     self.draw_welcome_screen(inputs, clock, virtual_frame_buffer);
        // } else if self.game {
        //     self.draw_game(virtual_frame_buffer);
        // } else if self.menu {
        //     self.draw_menu(virtual_frame_buffer);
        // }
        self.draw_welcome_screen(inputs, clock, virtual_frame_buffer);
    }

    fn update_welcome_screen(
        &mut self,
        inputs: &WinitInputHelper,
        _virtual_frame_buffer: &mut VirtualFrameBuffer,
    ) {

        /*---------------------------------------------------------- */
        //                 choosing default scenarios
        /*---------------------------------------------------------- */

        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.reset();
            self.set_state(false, false);
        } else if inputs.key_pressed(VirtualKeyCode::Space) {
            self.pause = !self.pause;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        } else if inputs.key_pressed(VirtualKeyCode::Key1) {
            self.reset();
            self.mandel_x_center = X_COORD;
            self.mandel_y_center = Y_COORD;
        } else if inputs.key_pressed(VirtualKeyCode::Key2) {
            self.reset();
            self.mandel_x_center = -0.749089134879074;
            self.mandel_y_center = 0.04575273713964573;
        } else if inputs.key_pressed(VirtualKeyCode::Key3) {
            self.reset();
            self.mandel_x_center = -1.254716173206939;
            self.mandel_y_center = -0.03269356495238624;
        } else if inputs.key_pressed(VirtualKeyCode::Key4) {
            self.reset();
            self.mandel_x_center = 0.26781837605081366;
            self.mandel_y_center = -0.003918849643395729;
        } else if inputs.key_pressed(VirtualKeyCode::Key5) {
            self.reset();
            self.mandel_x_center = -0.10971550489778131;
            self.mandel_y_center = 0.8945121343911098;
        } else if inputs.key_pressed(VirtualKeyCode::Key6) {
            self.reset();
            self.mandel_x_center = -1.403277422173161;
            self.mandel_y_center = -0.00022314715329581908;
        } else if inputs.key_pressed(VirtualKeyCode::Key7) {
            self.reset();
            self.mandel_x_center = -0.19827338980477996;
            self.mandel_y_center = -1.100975539162933;
        } else if inputs.key_pressed(VirtualKeyCode::Key8) {
            self.reset();
            self.mandel_x_center = -1.9425557680573255;
            self.mandel_y_center = 0.0; 
        } else if inputs.key_pressed(VirtualKeyCode::Key9) {
            self.reset();
            self.mandel_x_center = 0.3514237590616519;
            self.mandel_y_center = -0.06386655970753488;
        }

    	/*---------------------------------------------------------- */
        //                      Rendering controls 
        /*---------------------------------------------------------- */

        else if inputs.key_pressed(VirtualKeyCode::Slash) {
            self.reverse = !self.reverse;
        } else if inputs.key_pressed(VirtualKeyCode::R) {
            self.reset();
        } else if inputs.key_pressed(VirtualKeyCode::Comma) {
            self.max_iteration -= 10;
            if self.max_iteration < MIN_ITER {self.max_iteration = MIN_ITER};
            println!("max_iteration: {}", self.max_iteration);
        } else if inputs.key_pressed(VirtualKeyCode::Period) {
            self.max_iteration += 10;
            println!("max_iteration: {}", self.max_iteration);
        }

        /*---------------------------------------------------------- */
        //                      Movement controls 
        /*---------------------------------------------------------- */
        if inputs.key_pressed_os(VirtualKeyCode::Up) {
            self.mandel_y_center -= self.mandel_y_range/50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        }

        if inputs.key_pressed_os(VirtualKeyCode::Down) {
            self.mandel_y_center += self.mandel_y_range/50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        } 
        
        if inputs.key_pressed_os(VirtualKeyCode::Left) {
            self.mandel_x_center -= self.mandel_x_range/50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        } 
        
        if inputs.key_pressed_os(VirtualKeyCode::Right) {
            self.mandel_x_center += self.mandel_x_range/50.0;
            println!("x: {}, y: {}", self.mandel_x_center, self.mandel_y_center);
        }

    }

    fn draw_welcome_screen(
        &mut self,
        _inputs: &WinitInputHelper,
        clock: &Clock,
        virtual_frame_buffer: &mut VirtualFrameBuffer,
    ) {

        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.get_console_mut().display = false;
        virtual_frame_buffer.clear(BLACK);

        let mandel_x_min: f64 = self.mandel_x_center - self.mandel_x_range / 2.0;
        let mandel_y_min: f64 = self.mandel_y_center - self.mandel_y_range / 2.0;

        let mut max_iteration_count:usize = 0;
        let mut x0: f64;
        let mut y0: f64;
        let mut x: f64;
        let mut y: f64;
        let mut x2: f64;
        let mut y2: f64;
        let mut iteration: usize;

        // Mandelbrot algorithm from Wikipedia : https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set
        for py in 0..VIRTUAL_HEIGHT {
            for px in 0..VIRTUAL_WIDTH {

                x0 = ((px as f64 * self.mandel_x_range) / VIRTUAL_WIDTH as f64) + mandel_x_min;
                y0 = ((py as f64 * self.mandel_y_range) / VIRTUAL_HEIGHT as f64) + mandel_y_min;
                x = 0.0;
                y = 0.0;
                iteration = 0;

                while iteration < self.max_iteration {
                    x2 = x*x;
                    y2 = y*y;
                    if x2 + y2 > 4.0 {break}
                    y = (x + x) * y + y0;
                    x = x2 - y2 + x0;
                    iteration += 1;
                }

                let color: u8 = if iteration == self.max_iteration {
                    max_iteration_count += 1;
                    BLACK
                } else {
                    TREE_PALETTE[(iteration % TREE_PALETTE.len()) as usize]
                };
                
                virtual_frame_buffer.set_pixel(px, py, color);
            }
        }

        // Increasing the amount of details the deeper we get, to keep the screen filled
        let empty_ratio: f64 = max_iteration_count as f64 / (VIRTUAL_WIDTH * VIRTUAL_HEIGHT) as f64;
        let empty_ratio_delta: f64 = empty_ratio - self.previous_empty_ratio;

        // If the proportion of empty pixels reaches a certain threashold, 
        // and the delta between that render and the previous one increases,
        // then increase numer of iterations by 1, to draw more.
        if empty_ratio >= EMPTY_RATIO_TRIGGER && empty_ratio_delta >= EMPTY_RATIO_DELTA_TRIGGER {
            self.max_iteration += 1;

            // If one more iteration is not enough to reduce the delta, add one more iteration
            if empty_ratio_delta - self.previous_empty_ratio_delta >= EMPTY_RATIO_DELTA_TRIGGER {
                self.max_iteration += 1;
            }
        } 

        // If to many pixels are drawn, reduce detail
        // Useful when zooming out, or when going from an empty region to a dense one.
        // multpiplied by 1.1 to avoid flickering
        if empty_ratio < EMPTY_RATIO_TRIGGER * 1.1 {
            self.max_iteration -= 1;
            if self.max_iteration < MIN_ITER {self.max_iteration = MIN_ITER};
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
        if empty_ratio == 1.0 {self.reset()};

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
}

