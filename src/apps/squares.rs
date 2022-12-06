use std::time::Instant;

use app_macro::*;
use app_macro_derive::AppMacro;
use virtual_frame_buffer::{*, config::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH}};
use rand::Rng;

use winit::{
    event::{KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::ControlFlow,
};

#[derive(AppMacro)]
pub struct Squares {
    is_shell: bool,
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    draw_square: bool,
    last_update: Instant
}

impl Squares {
    pub fn new() -> Squares {
        Squares {
            is_shell: false,
            name: String::from("squares"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            draw_square: true,
            last_update: Instant::now()
        }
    }

    pub fn update_app(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        let mut response = AppResponse::new();

        virtual_frame_buffer.get_console_mut().display = false;

        match char_received {
            Some(_c) => (),
            None => ()
        }

        let now = Instant::now();
        if now.duration_since(self.last_update).as_millis() >= 500 {
            self.draw_square = true;
            self.last_update = Instant::now();
        }

        return Some(response);
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_console_mut().display = false;
        virtual_frame_buffer.get_text_layer_mut().clear();

        if self.draw_square {

            let mut random = rand::thread_rng();

            let x: usize = random.gen_range(0..VIRTUAL_WIDTH);
            let y: usize = random.gen_range(0..VIRTUAL_HEIGHT);
            let width: usize = random.gen_range(0..(VIRTUAL_WIDTH - x));
            let height: usize = random.gen_range(0..(VIRTUAL_HEIGHT - y));
            let color: u8 = random.gen_range(0..32);
            let fill = if random.gen_range(0..2) == 0 {
                true
            } else {
                false
            };

            draw_a_square(x, y, width, height, color, fill, virtual_frame_buffer.get_frame_mut());
            self.draw_square = false;
        }
    }
}
