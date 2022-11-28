use app_macro::*;
use app_macro_derive::AppMacro;
use winit::event_loop::ControlFlow;

use virtual_frame_buffer::*;
use rand::Rng;
use winit::event::{KeyboardInput, VirtualKeyCode};

#[derive(AppMacro)]
pub struct Squares {
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    draw_appa_line: bool,
}

impl Squares {
    pub fn new() -> Squares {
        Squares {
            name: String::from("Squares"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            draw_appa_line: true,
        }
    }

    pub fn update_app(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
        virtual_frame_buffer: &mut VirtualFrameBuffer
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
                                self.draw_appa_line = true;
                            }
                            _ => (),
                        }
                    }
                    None => (),
                }
            }
            None => (),
        }

        match char_received {
            Some(_c) => (),
            None => ()
        }

        return response;
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        //virtual_frame_buffer.get_text_layer().clear();
        //virtual_frame_buffer.get_text_layer().show_cursor = false;

        let max_x = virtual_frame_buffer.get_width();
        let max_y = virtual_frame_buffer.get_height();

        let mut random = rand::thread_rng();

        for _i in 0..5 {
            let pos_x: usize = random.gen_range(0..max_x);
            let pos_y: usize = random.gen_range(0..max_y);
            let width: usize = random.gen_range(0..max_x);
            let height: usize = random.gen_range(0..max_y);
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
                width,
                height,
                fill,
                color,
            };
            draw_square(square, virtual_frame_buffer.get_frame_mut());
        }
    }
}
