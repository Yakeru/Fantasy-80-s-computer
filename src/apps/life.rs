use app_macro::{AppMacro, AppResponse};
use app_macro_derive::AppMacro;
use rand::Rng;
use virtual_frame_buffer::{VirtualFrameBuffer, config::{TEXT_COLUMNS, TEXT_ROWS}, color_palettes::{WHITE, BLACK, DARKGREY}};
use winit::event::KeyboardInput;

#[derive(AppMacro)]
pub struct Life {
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    gen_a: Box<[[u8; TEXT_COLUMNS]; TEXT_ROWS]>,
    gen_b: Box<[[u8; TEXT_COLUMNS]; TEXT_ROWS]>,
    init: bool
}

impl Life {
    pub fn new() -> Life {
        Life {
            name: String::from("Life"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            gen_a: Box::new([[0; TEXT_COLUMNS]; TEXT_ROWS]),
            gen_b: Box::new([[0; TEXT_COLUMNS]; TEXT_ROWS]),
            init: false
        }
    }

    pub fn update_app(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {

        if !self.init {

            virtual_frame_buffer.get_console_mut().display = false;

            let mut random = rand::thread_rng();

            for col in 0..TEXT_COLUMNS {
                for row in 0..TEXT_ROWS {
            
                    let alive: u8 = random.gen_range(0..2);
                    if alive > 0 {
                        self.gen_a[row][col] = 1;
                    }
                }
            }

            self.init = true;
        }

        //Update on half second tick
        if virtual_frame_buffer.get_clock().1 == true {

            //if hals second latched, update gen_a into gen_b and display gen_b
            //else update gen_b into gen_a and display gen_a
            if virtual_frame_buffer.get_clock().2 == true {
                for col in 0..TEXT_COLUMNS {
                    for row in 0..TEXT_ROWS {
                        //Check all surrounding cells and count how many are alive
                        //if < 2, cell dies
                        //if > 3, cell dies
                        //if == 3, cell is born
                        let mut count = 0;
                        for i in if col > 0 { col-1 } else { 0 }..if col < TEXT_COLUMNS - 1 { col + 1 } else { TEXT_COLUMNS } {
                            for j in if row > 0 { row-1 } else { 0 }..if row < TEXT_ROWS - 1 { row + 1 } else { TEXT_ROWS } {
                                if i != col && j != row && self.gen_a[row][col] > 0 { count += 1 };
                            }
                        }

                        if count < 2 || count > 3 {
                            self.gen_b[row][col] = 0
                        }

                        if count == 3 {
                            self.gen_b[row][col] = 1
                        }
                    }
                }
            } else {
                for col in 0..TEXT_COLUMNS {
                    for row in 0..TEXT_ROWS {
                        //Check all surrounding cells and count how many are alive
                        //if < 2, cell dies
                        //if > 3, cell dies
                        //if == 3, cell is born
                        let mut count = 0;
                        for i in if col > 0 { col-1 } else { 0 }..if col < TEXT_COLUMNS - 1 { col + 1 } else { TEXT_COLUMNS } {
                            for j in if row > 0 { row-1 } else { 0 }..if row < TEXT_ROWS - 1 { row + 1 } else { TEXT_ROWS } {
                                if i != col && j != row && self.gen_b[row][col] > 0 { count += 1 };
                            }
                        }

                        if count < 2 || count > 3 {
                            self.gen_a[row][col] = 0
                        }

                        if count == 3 {
                            self.gen_a[row][col] = 1
                        }
                    }
                }
            }
            
        }

        return None;
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.clear_frame_buffer(DARKGREY);
        
        //render gen_b else render gen_a
        if virtual_frame_buffer.get_clock().2 == true {
            for col in 0..TEXT_COLUMNS {
                for row in 0..TEXT_ROWS {
                    if self.gen_b[row][col] > 0 {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, '*', Some(WHITE), Some(BLACK), false, false, false);
                    } else {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, ' ', Some(WHITE), Some(BLACK), false, false, false);
                    }
                }
            }
        } else {
            for col in 0..TEXT_COLUMNS {
                for row in 0..TEXT_ROWS {
                    if self.gen_a[row][col] > 0 {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, '*', Some(WHITE), Some(BLACK), false, false, false);
                    } else {
                        virtual_frame_buffer.get_text_layer_mut().insert_char_xy(col, row, ' ', Some(WHITE), Some(BLACK), false, false, false);
                    }
                }
            }
        }
    }
}