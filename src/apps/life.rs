use std::time::Instant;

use app_macro_derive::AppMacro;
use rand::Rng;
use virtual_frame_buffer::{VirtualFrameBuffer, config::{TEXT_COLUMNS, TEXT_ROWS}, color_palettes::*};

#[derive(AppMacro)]
pub struct Life {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    gen_a: Box<[[u8; TEXT_COLUMNS]; TEXT_ROWS]>,
    gen_b: Box<[[u8; TEXT_COLUMNS]; TEXT_ROWS]>,
    toggle_gen: bool,
    last_update: Instant,
    welcome_screen: bool,
    game: bool,
    menu: bool,
    alive: bool
}

impl Life {
    pub fn new() -> Life {
        Life {
            enable_auto_escape: false,
            name: String::from("life"),
            updating: false,
            drawing: false,
            initialized: false,
            gen_a: Box::new([[0; TEXT_COLUMNS]; TEXT_ROWS]),
            gen_b: Box::new([[0; TEXT_COLUMNS]; TEXT_ROWS]),
            toggle_gen: true,
            last_update: Instant::now(),
            alive: true,
            welcome_screen: true,
            game: false,
            menu: false
        }
    }

    // Randomizes gen_a. gen_B is emptied,
    // Sets everything back to show gen_a and calculate gen_b
    pub fn init_app(&mut self, _virtual_frame_buffer: &mut VirtualFrameBuffer) {
        self.welcome_screen = true;
        self.game = false;
        self.menu = false;
        self.restart_sim();
    }

    pub fn update_app(
        &mut self,
        inputs: &WinitInputHelper,
        _clock: &Clock,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        
        if self.welcome_screen {
            self.update_welcome_screen(inputs, virtual_frame_buffer);
        } else if self.game {
            self.update_game(inputs, virtual_frame_buffer);
        } else {
            self.update_menu(inputs, virtual_frame_buffer);
        }

        return None;
    }

    pub fn draw_app(&mut self, inputs: &WinitInputHelper, clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        if self.welcome_screen {
            self.draw_welcome_screen(inputs, clock, virtual_frame_buffer);
        } else if self.game {
            self.draw_game(virtual_frame_buffer);
        } else if self.menu {
            self.draw_menu(virtual_frame_buffer);
        }
    }

    fn restart_sim(&mut self) {
        self.gen_b = Box::new([[0; TEXT_COLUMNS]; TEXT_ROWS]);

        let mut random = rand::thread_rng();

        for row in 0..TEXT_ROWS {
            for col in 0..TEXT_COLUMNS {
                self.gen_a[row][col] = random.gen_range(0..2);
            }
        }

        self.alive = true;
        self.toggle_gen = true;
    }

    fn draw_welcome_screen(&mut self, _inputs: &WinitInputHelper, clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.get_console_mut().display = false;
        virtual_frame_buffer.clear_frame_buffer(BLACK);
        if clock.second_latch  && clock.half_second_latch {
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 10, " *  C n a '  G m  O  L f   * ", Some(BLUE), Some(BLACK), false, false, false);
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 11, " *   o w y s  a e  f  i e  * ", Some(BLUE), Some(BLACK), false, false, false);
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 12, " *                         * ", Some(BLUE), Some(BLACK), false, false, false);
        } else if clock.second_latch  && !clock.half_second_latch {
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 11, "*** Conway's Game Of Life ***", Some(BLUE), Some(BLACK), false, false, false);
        } else if !clock.second_latch  && clock.half_second_latch {
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 10, " *                         * ", Some(BLUE), Some(BLACK), false, false, false);
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 11, " *  C n a '  G m  O  L f   * ", Some(BLUE), Some(BLACK), false, false, false);
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 12, " *   o w y s  a e  f  i e  * ", Some(BLUE), Some(BLACK), false, false, false);
        } else {
            virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 29)/2, 11, "*** Conway's Game Of Life ***", Some(BLUE), Some(BLACK), false, false, false);
        }
        virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 20)/2, 20, "Press SPACE to start", Some(ORANGE), Some(BLACK), false, true, false);
        virtual_frame_buffer.get_text_layer_mut().insert_string_xy((TEXT_COLUMNS - 24)/2, TEXT_ROWS - 1, "2022 - Damien Torreilles", Some(TRUE_BLUE), Some(BLACK), false, false, false);
    }

    fn update_welcome_screen(&mut self, inputs: &WinitInputHelper, _virtual_frame_buffer: &mut VirtualFrameBuffer) {

        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.set_state(false, false);
        } else if !inputs.text().is_empty() {
            self.welcome_screen = false;
            self.menu = false;
            self.game = true;
        }
    }

    fn draw_game(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.get_console_mut().display = false;
        virtual_frame_buffer.clear_frame_buffer(WHITE);

        let bkg_color = Some(BLACK);

        let colors = [RED, DARK_ORANGE, ORANGE, YELLOW, LIGHT_YELLOW, WHITE];
        let chars = ['🯆','🯅','🯇','🯈'];
        //render gen_a else render gen_b
        if self.toggle_gen {
            for col in 0..TEXT_COLUMNS {
                for row in 0..TEXT_ROWS {
                    if self.gen_a[row][col] > 0 {
                        let color = Some(colors[(self.gen_a[row][col] % (colors.len() - 1) as u8) as usize ]);
                        let char = chars[(self.gen_a[row][col] % (chars.len() - 1) as u8) as usize ];
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, char, color, bkg_color, false, false, false);
                    } else {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, ' ', bkg_color, bkg_color, false, false, false);
                    }
                }
            }
        } else {
            for col in 0..TEXT_COLUMNS {
                for row in 0..TEXT_ROWS {
                    if self.gen_b[row][col] > 0 {
                        let color = Some(colors[(self.gen_a[row][col] % (colors.len() - 1) as u8) as usize]);
                        let char = chars[(self.gen_a[row][col] % (chars.len() - 1) as u8) as usize ];
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, char, color, bkg_color, false, false, false);
                    } else {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, ' ', bkg_color, bkg_color, false, false, false);
                    }
                }
            }
        }
    }

    fn update_game(&mut self, inputs: &WinitInputHelper, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        if inputs.key_released(VirtualKeyCode::C) {
            self.restart_sim();
        }

        if inputs.key_released(VirtualKeyCode::Escape) {
            self.init_app(virtual_frame_buffer);
        }

        let now = Instant::now();

        if now.duration_since(self.last_update).as_millis() >= 50 {
            // Calculate gen_b from gen_a, else calculate gen_b from gen_a
            if self.toggle_gen {
                self.alive = calculate_life(&mut self.gen_a, &mut self.gen_b);
                self.toggle_gen = !self.toggle_gen;
            } else {
                self.alive = calculate_life(&mut self.gen_b, &mut self.gen_a);
                self.toggle_gen = !self.toggle_gen;
            }

            self.last_update = Instant::now();

            if !self.alive {
                self.restart_sim();
            }
        }
    }

    fn draw_menu(&mut self, _virtual_frame_buffer: &mut VirtualFrameBuffer) {

    }

    fn update_menu(&mut self, inputs: &WinitInputHelper, _virtual_frame_buffer: &mut VirtualFrameBuffer) {

        if inputs.key_released(VirtualKeyCode::Escape) {
            self.welcome_screen = true;
            self.menu = false;
            self.game = false;
        }
    }

}

/// Conway's Game of Life
/// Returns false if stuck in infinite loop, true if things are still dying and birthing
fn calculate_life(current_gen: &mut [[u8; TEXT_COLUMNS]; TEXT_ROWS], next_gen: &mut [[u8; TEXT_COLUMNS]; TEXT_ROWS]) -> bool {
    let mut death_count = 0;
    let mut birth_count = 0;

    for row in 0..TEXT_ROWS {
        for col in 0..TEXT_COLUMNS {
            let mut count = 0;
            for row_test in (if row == 0 {0} else {row-1})..(if row == TEXT_ROWS - 1 {TEXT_ROWS - 1} else {row+2}) {
                for col_test in (if col == 0 {0} else {col-1})..(if col == TEXT_COLUMNS - 1 {TEXT_COLUMNS - 1} else {col+2}) {
                    if !(col_test == col && row_test == row) && current_gen[row_test][col_test] > 0 { 
                        count += 1 
                    };
                }
            }
            if count < 2 || count > 3 {
                next_gen[row][col] = 0;
                if current_gen[row][col] == 1 {
                    death_count += 1;
                }
            } else if count == 3 && current_gen[row][col] == 0 {
                next_gen[row][col] = 1;
                birth_count += 1;
            } else {
                next_gen[row][col] = if current_gen[row][col] == 0 {0} else {current_gen[row][col] + 1}
            }
        }
    }
    //println!("{}, {}", birth_count, death_count);
    if death_count == 0 && birth_count == 0 {false} else {true}
}