use app_macro::*;
use app_macro_derive::AppMacro;
use winit::event_loop::ControlFlow;

use crate::virtual_frame_buffer::*;
use rand::Rng;
use winit::dpi::PhysicalSize;
use winit::event::{KeyboardInput, VirtualKeyCode};

#[derive(AppMacro)]
pub struct Squares {
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    draw_a_line: bool,
}

impl Squares {
    pub fn new() -> Squares {
        Squares {
            name: String::from("Squares"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            draw_a_line: true,
        }
    }

    pub fn update(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
    ) -> AppResponse {
        let mut response = AppResponse::new();

        if !self.started {
            self.start();
        }

        match keybord_input {
            Some(key) => {
                match key.virtual_keycode {
                    Some(code) => {
                        match code {
                            VirtualKeyCode::Escape => {
                                //Escape
                                response.set_message("Escape key pressed".to_string());
                                response.event = Some(ControlFlow::ExitWithCode(0));
                                self.end();
                            }

                            VirtualKeyCode::Return => {
                                //Enter
                                self.draw_a_line = true;
                            }
                            _ => (),
                        }
                    }
                    None => (),
                }
            }
            None => (),
        }

        return response;
    }

    pub fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        //virtual_frame_buffer.get_text_layer().clear();
        //virtual_frame_buffer.get_text_layer().show_cursor = false;

        let max_x = virtual_frame_buffer.get_width();
        let max_y = virtual_frame_buffer.get_height();

        let mut random = rand::thread_rng();

        for _i in 0..5 {
            let pos_x: usize = random.gen_range(0..max_x);
            let pos_y: usize = random.gen_range(0..max_y);
            let size = PhysicalSize::new(random.gen_range(0..max_x), random.gen_range(0..max_y));
            let color: u8 = random.gen_range(0..32);
            let fill = if random.gen_range(0..4) > 2 {
                true
            } else {
                false
            };
            //if color >= 2 {color = 28} else {color = 0};

            let square: Square = Square {
                pos_x,
                pos_y,
                size,
                fill,
                color,
            };
            virtual_frame_buffer.draw_square(square);
        }
    }
}
