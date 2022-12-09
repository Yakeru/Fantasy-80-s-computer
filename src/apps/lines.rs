use virtual_frame_buffer::*;
use app_macro::*;
use app_macro_derive::AppMacro;
use rand::Rng;
use winit::event::{VirtualKeyCode, ElementState};

#[derive(AppMacro)]
pub struct Lines {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    draw_line: bool,
    clear: bool
}

impl Lines {
    pub fn new() -> Lines {
        Lines {
            enable_auto_escape: true,
            name: String::from("lines"),
            updating: false,
            drawing: false,
            initialized: false,
            draw_line: true,
            clear: false
        }
    }

    pub fn init_app(&mut self, _virtual_frame_buffer: &mut VirtualFrameBuffer) {}

    pub fn update_app(
        &mut self,
        app_message: AppMessage,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        
        virtual_frame_buffer.get_console_mut().display = false;

        match app_message.char_received {
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

        return None;
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        let max_x = virtual_frame_buffer.get_width();
        let max_y = virtual_frame_buffer.get_height();

        virtual_frame_buffer.get_console_mut().display = false;
        virtual_frame_buffer.get_text_layer_mut().clear();
        //virtual_frame_buffer.get_text_layer().show_cursor = false;

        let mut random = rand::thread_rng();

        if self.draw_line {
            let x1: usize = random.gen_range(0..max_x);
            let y1: usize = random.gen_range(0..max_y);
            let x2: usize = random.gen_range(0..max_x);
            let y2: usize = random.gen_range(0..max_y);
            let color: u8 = random.gen_range(0..32);
            //if color >= 2 {color = 28} else {color = 0};

            let line: Line = Line {
                x1,
                y1,
                x2,
                y2,
                color,
            };
            draw_line(line, virtual_frame_buffer.get_frame_mut());
        }

        if self.clear {
            virtual_frame_buffer.clear_frame_buffer(0);
            self.clear = false;
        }
    }
}
