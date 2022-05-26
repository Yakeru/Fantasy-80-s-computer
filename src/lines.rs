use crate::process::*;
use crate::virtual_frame_buffer::*;
use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use winit::dpi::PhysicalSize;
use crate::text_layer::TextLayerChar;
use rand::Rng;

pub struct Lines {
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    draw_a_line: bool,
}

impl Lines {
    pub fn new() -> Lines {
        Lines {
            name: String::from("Lines"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            draw_a_line: true
        }
    }
}

impl Process for Lines {
    fn start(&mut self){
        self.started = true;
    }

    fn end(&mut self) {
        self.ended = true;
    }

    fn update(&mut self, character_received: Option<char>, key_pressed_os: Option<VirtualKeyCode>, key_released: Option<VirtualKeyCode>) -> ProcessResponse {

        let mut response = ProcessResponse::new();

        if !self.started {
            self.start();
        }

        match character_received {
            Some(c) => {
                match c {
                    '\u{001B}'  => { //Escape
                        self.updating = false;
                        self.drawing = false;
                        self.end();
                    }

                    '\u{000D}' => { //Enter
                        self.draw_a_line = true;
                    }
                    
                    _ => ()
                }
            }
            None => ()
        }

        return response;
    }

    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        let max_x = virtual_frame_buffer.get_width();
        let max_y = virtual_frame_buffer.get_height();

        let mut random = rand::thread_rng();

        for _i in 0..100 {
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
                color
            };
            virtual_frame_buffer.draw_line(line);
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_state(&mut self, updating: bool, drawing: bool) {
        self.updating = updating;
        self.drawing = drawing;

        if drawing {self.updating = true}
        if !updating {self.drawing = false}
    }

    fn get_state(&self) -> (bool, bool) {
        return (self.updating, self.drawing)
    }
}