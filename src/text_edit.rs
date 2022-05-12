use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use crate::text_layer::TextLayerChar;
use std::io::{self, Write};
use crate::app::*;
use crate::virtual_frame_buffer::VirtualFrameBuffer;

const DEFAULT_BKG_COLOR: u8 = 0;
const DEFAULT_COLOR: u8 = 1;

pub struct TextEdit {
    pub app: App,
    selected_color: u8,
    selected_bkg_color: u8,
    columns: u8,
    rows: u8,
    buffer: Vec<TextLayerChar>
}

impl TextEdit {

    pub fn new(pid: usize) -> TextEdit {

        // text_layer.clear();
        // text_layer.push_char('_', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false); //re insert cursor

        let app = App::new(String::from("Yak's Text Editor"), pid);
        let buffer = Vec::new();

        TextEdit {
            app,
            selected_color: DEFAULT_COLOR,
            selected_bkg_color: DEFAULT_BKG_COLOR,
            columns: 0,
            rows: 0,
            buffer
        }
    }
}

impl Update for TextEdit {

    fn start(&mut self){}

    fn end(&mut self) {
        self.app.started = false;
        self.app.drawing = false;
        self.app.updating = false;
        self.app.ended = true;
    }

    fn update(&mut self, character_received: Option<char>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow> {

        if !self.app.started {
            self.start();
            self.app.started = true;
        }

        match character_received {
            Some(c) => {
                match c as u8 {
                    8 => { //Backspace
                        self.buffer.pop();
                    } 
                    
                    13 => { //Enter
                        
                    }
                    
                    27 => { //Escape
                        //self.end();
                    }
                    
                    _ => {
                        let plop: TextLayerChar = TextLayerChar {
                            c,
                            color: self.selected_color,
                            background_color: self.selected_bkg_color,
                            blink: false,
                            flipp: false
                        };
                        
                        self.buffer.push(plop);
                    }
                }

            }
            None => ()
        }

        match key_released {
            Some(k) => {
                match k {
                    VirtualKeyCode::Left => {
                        if self.selected_color == 7 {self.selected_color = 0} else {self.selected_color += 1}
                    }
        
                    VirtualKeyCode::Right => {
                        if self.selected_color == 0 {self.selected_color = 7} else {self.selected_color -= 1}
                    }
        
                    VirtualKeyCode::Up => {
                        if self.selected_bkg_color == 7 {self.selected_bkg_color = 0} else {self.selected_bkg_color += 1}
                    }
        
                    VirtualKeyCode::Down => {
                        if self.selected_bkg_color == 0 {self.selected_bkg_color = 7} else {self.selected_bkg_color -= 1}
                    }
        
                    VirtualKeyCode::PageUp => {
                        // self.text_layer.scroll_up();
                    }

                    _ => () 
                }
            }
            None => ()
        }

        return None;

    }
}

impl Draw for TextEdit {
    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        virtual_frame_buffer.get_text_layer().clear();

        for text_layer_char in self.buffer.chunks_exact_mut(1) {
            virtual_frame_buffer.get_text_layer().push_character(Some(text_layer_char[0]));
        }

        virtual_frame_buffer.get_text_layer().push_char('_', self.selected_color, self.selected_bkg_color, false);
    }
}