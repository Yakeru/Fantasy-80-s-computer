use crate::process::*;
use crate::virtual_frame_buffer::VirtualFrameBuffer;
use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use winit::dpi::PhysicalSize;
use crate::text_layer::TextLayerChar;

const DEFAULT_BKG_COLOR: u8 = 0;
const SPRITE_SIZE: PhysicalSize<usize> = PhysicalSize::new(16, 16);
const EDITOR_PIXEL_SIZE: PhysicalSize<usize> = PhysicalSize::new(10, 10);

pub struct SpriteEditor {
    updating: bool,
    drawing: bool,
    pixel_grid: [u8; SPRITE_SIZE.width * SPRITE_SIZE.height],
}

#[derive(Copy, Clone)]
struct Square {
    pos_x: usize,
    pos_y: usize,
    size: PhysicalSize<usize>,
    color: u8,
    fill: bool
}

impl SpriteEditor {

    pub fn new() -> SpriteEditor {
        SpriteEditor {
            updating: false,
            drawing: false,
            pixel_grid: [0; SPRITE_SIZE.width * SPRITE_SIZE.height]
        }
    }

    fn draw_square(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer, square: Square) {

        let start_offset: usize = virtual_frame_buffer.get_width() * square.pos_y + square.pos_x;
        let mut pixel_count = 0;

        for row in 0..square.size.width {
            for column in 0..square.size.height {
                let offset = start_offset + column + virtual_frame_buffer.get_width() * row;
                virtual_frame_buffer.get_frame()[offset] = square.color;
            }
        }
    }
}

impl Process for SpriteEditor {

    fn start(&mut self){}

    fn end(&mut self) {}

    fn update(&mut self, character_received: Option<char>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow> {

        // if !self.app.started {
        //     self.start();
        //     self.app.started = true;
        // }

        match character_received {
            Some(c) => {
                match c as u8 {
                    8 => { //Backspace  
                    } 
                    
                    13 => { //Enter
                    }
                    
                    27 => { //Escape 
                    }
                    
                    _ => {  
                    }
                }
            }
            None => ()
        }

        match key_released {
            Some(k) => {
                match k {
                    VirtualKeyCode::Left => {
                        
                    }
        
                    VirtualKeyCode::Right => {
                        
                    }
        
                    VirtualKeyCode::Up => {
                        
                    }
        
                    VirtualKeyCode::Down => {
                        
                    }
        
                    VirtualKeyCode::PageUp => {

                    }

                    _ => () 
                }
            }
            None => ()
        }

        return None;

    }

    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
        virtual_frame_buffer.get_text_layer().clear();

        //Drawing are Background square
        let bkg_square_width = SPRITE_SIZE.width * EDITOR_PIXEL_SIZE.width + SPRITE_SIZE.width + 3;
        let bkg_square_height = SPRITE_SIZE.height * EDITOR_PIXEL_SIZE.height + SPRITE_SIZE.height + 3;
        let square_size: PhysicalSize<usize> = PhysicalSize::new(bkg_square_width, bkg_square_height);

        let bkg_square: Square = Square {
            pos_x: 20,
            pos_y: 20,
            size: square_size,
            color: 5,
            fill: true
        };

        self.draw_square(virtual_frame_buffer, bkg_square);

        //Pixels
        for row in 0..SPRITE_SIZE.height {
            for column in 0..SPRITE_SIZE.width {

                let pos_x: usize = column * EDITOR_PIXEL_SIZE.width + column + bkg_square.pos_x + 2;
                let pos_y: usize = row * EDITOR_PIXEL_SIZE.height + row  + bkg_square.pos_y + 2;

                let pixel_square: Square = Square {
                    pos_x,
                    pos_y,
                    size: EDITOR_PIXEL_SIZE,
                    color: 0,
                    fill: true
                };

                self.draw_square(virtual_frame_buffer, pixel_square);
            }
        }



    }

    fn get_name(&mut self) -> &str {
        return "Sprite Editor";
    }

    fn set_state(&mut self, updating: bool, drawing: bool) {
        self.updating = updating;
        self.drawing = drawing;

        if drawing {self.updating = true}
        if !updating {self.drawing = false}
    }

    fn get_state(&mut self) -> (bool, bool) {
        return (self.updating, self.drawing)
    }
}