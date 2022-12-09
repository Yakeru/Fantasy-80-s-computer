use std::time::Instant;

use app_macro::{AppMacro, AppResponse, AppMessage};
use app_macro_derive::AppMacro;
use rand::Rng;
use virtual_frame_buffer::{VirtualFrameBuffer, config::{TEXT_COLUMNS, TEXT_ROWS}, color_palettes::*};
use winit::event::{VirtualKeyCode, ElementState};

#[derive(AppMacro)]
pub struct Life {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    gen_a: Box<[[u8; TEXT_COLUMNS]; TEXT_ROWS]>,
    gen_b: Box<[[u8; TEXT_COLUMNS]; TEXT_ROWS]>,
    init: bool,
    toggle_gen: bool,
    last_update: Instant,
    alive: bool
}

impl Life {
    pub fn new() -> Life {
        Life {
            enable_auto_escape: true,
            name: String::from("life"),
            updating: false,
            drawing: false,
            initialized: false,
            gen_a: Box::new([[0; TEXT_COLUMNS]; TEXT_ROWS]),
            gen_b: Box::new([[0; TEXT_COLUMNS]; TEXT_ROWS]),
            init: true,
            toggle_gen: true,
            last_update: Instant::now(),
            alive: true
        }
    }

    // Randomizes gen_a. gen_B is emptied,
    // Sets everything back to show gen_a and calculate gen_b
    pub fn init_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_console_mut().display = false;

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

    pub fn update_app(
        &mut self,
        app_message: AppMessage,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        
        // Clear and re-start if 'c' is pressed
        match app_message.char_received {
            Some(char) => {
                if char == 'c' {
                    self.init_app(virtual_frame_buffer);
                }
            },
            _ => ()
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
                self.init = true;
            }
        }

        return None;
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.get_console_mut().display = false;
        virtual_frame_buffer.clear_frame_buffer(WHITE);

        let bkg_color = Some(BLACK);

        let colors = [RED, DARK_ORANGE, ORANGE, YELLOW, LIGHT_YELLOW, WHITE];
        let len = colors.len() - 1;
        //render gen_a else render gen_b
        if self.toggle_gen {
            for col in 0..TEXT_COLUMNS {
                for row in 0..TEXT_ROWS {
                    if self.gen_a[row][col] > 0 {
                        let color = Some(colors[(self.gen_a[row][col] % len as u8) as usize ]);
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, '*', color, bkg_color, false, false, false);
                    } else {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, ' ', bkg_color, bkg_color, false, false, false);
                    }
                }
            }
        } else {
            for col in 0..TEXT_COLUMNS {
                for row in 0..TEXT_ROWS {
                    if self.gen_b[row][col] > 0 {
                        let color = Some(colors[(self.gen_a[row][col] % len as u8) as usize]);
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, '*', color, bkg_color, false, false, false);
                    } else {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, ' ', bkg_color, bkg_color, false, false, false);
                    }
                }
            }
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