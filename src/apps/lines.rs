use virtual_frame_buffer::*;
use app_macro::*;
use app_macro_derive::AppMacro;
use rand::Rng;
use winit::{
    event::{KeyboardInput, VirtualKeyCode},
    event_loop::ControlFlow,
};

#[derive(AppMacro)]
pub struct Lines {
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    draw_line: bool,
    clear: bool
}

impl Lines {
    pub fn new() -> Lines {
        Lines {
            name: String::from("Lines"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            draw_line: true,
            clear: false
        }
    }

    pub fn update_app(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>
    ) -> AppResponse {
        let mut response = AppResponse::new();

        if !self.started {
            self.start();
        }

        match char_received {
            Some(unicode) => {
                match unicode {
                    // unicode::ENTER => {
                    //     self.draw_appa_line = true;
                    // },
                    'c' => {
                        self.clear = true;
                    }

                    _ => ()
                }
            }
            None => (),
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

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        let max_x = virtual_frame_buffer.get_width();
        let max_y = virtual_frame_buffer.get_height();

        virtual_frame_buffer.get_text_layer().clear();
        //virtual_frame_buffer.get_text_layer().show_cursor = false;

        let mut random = rand::thread_rng();

        if self.draw_line {
            let start_x: usize = random.gen_range(0..max_x);
            let start_y: usize = random.gen_range(0..max_y);
            let end_x: usize = random.gen_range(0..max_x);
            let end_y: usize = random.gen_range(0..max_y);
            let color: u8 = random.gen_range(0..32);
            //if color >= 2 {color = 28} else {color = 0};

            let line: Line = Line {
                start_x,
                start_y,
                end_x,
                end_y,
                color,
            };
            virtual_frame_buffer.draw_appline(line);
            // self.draw_appa_line = false;
        }

        if self.clear {
            virtual_frame_buffer.clear_frame_buffer(0);
            self.clear = false;
        }
    }
}
