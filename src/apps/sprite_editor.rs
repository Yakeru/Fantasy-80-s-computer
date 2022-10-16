use app_macro::*;
use app_macro_derive::AppMacro;

use crate::text_layer::TextLayerChar;
use crate::virtual_frame_buffer::*;
use winit::dpi::PhysicalSize;
use winit::{event::VirtualKeyCode, event_loop::ControlFlow};

const DEFAULT_BKG_COLOR: u8 = 0;
const SPRITE_SIZE: PhysicalSize<usize> = PhysicalSize::new(16, 16);
const EDITOR_PIXEL_SIZE: PhysicalSize<usize> = PhysicalSize::new(10, 10);
const EDITOR_PIXEL_HIGHLIGHT_SIZE: PhysicalSize<usize> =
    PhysicalSize::new(EDITOR_PIXEL_SIZE.width + 2, EDITOR_PIXEL_SIZE.height + 2);

#[derive(AppMacro)]
pub struct SpriteEditor {
    name: String,
    updating: bool,
    drawing: bool,
    pixel_grid: [u8; SPRITE_SIZE.width * SPRITE_SIZE.height],
    selected_pixel_x: usize,
    selected_pixel_y: usize,
    started: bool,
    ended: bool,
}

impl SpriteEditor {
    pub fn new() -> SpriteEditor {
        SpriteEditor {
            name: String::from("spriteEdit"),
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            pixel_grid: [0; SPRITE_SIZE.width * SPRITE_SIZE.height],
            selected_pixel_x: 0,
            selected_pixel_y: 0,
        }
    }

    fn update(
        &mut self,
        character_received: Option<char>,
        key_pressed_os: Option<VirtualKeyCode>,
        key_released: Option<VirtualKeyCode>,
    ) -> AppResponse {
        let mut response = AppResponse::new();

        if !self.started {
            self.start();
            self.started = true;
        }

        match character_received {
            Some(c) => {
                match c {
                    '\u{0008}' => { //Backspace
                    }

                    '\u{000D}' => { //Enter
                    }

                    '\u{001B}' => { //Escape
                    }

                    _ => {}
                }
            }
            None => (),
        }

        match key_pressed_os {
            Some(k) => match k {
                VirtualKeyCode::Left => {
                    if self.selected_pixel_x == 0 {
                    } else {
                        self.selected_pixel_x -= 1;
                    }
                }

                VirtualKeyCode::Right => {
                    if self.selected_pixel_x == SPRITE_SIZE.width - 1 {
                        self.selected_pixel_x = SPRITE_SIZE.width - 1
                    } else {
                        self.selected_pixel_x += 1;
                    }
                }

                VirtualKeyCode::Up => {
                    if self.selected_pixel_y == 0 {
                    } else {
                        self.selected_pixel_y -= 1;
                    }
                }

                VirtualKeyCode::Down => {
                    if self.selected_pixel_y == SPRITE_SIZE.height - 1 {
                        self.selected_pixel_y = SPRITE_SIZE.height - 1
                    } else {
                        self.selected_pixel_y += 1;
                    }
                }

                VirtualKeyCode::PageUp => {}

                _ => (),
            },
            None => (),
        }

        return response;
    }

    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
        virtual_frame_buffer.get_text_layer().clear();

        //Drawing are Background square
        let bkg_square_width = SPRITE_SIZE.width * EDITOR_PIXEL_SIZE.width + SPRITE_SIZE.width + 3;
        let bkg_square_height =
            SPRITE_SIZE.height * EDITOR_PIXEL_SIZE.height + SPRITE_SIZE.height + 3;
        let square_size: PhysicalSize<usize> =
            PhysicalSize::new(bkg_square_width, bkg_square_height);

        let bkg_square: Square = Square {
            pos_x: 20,
            pos_y: 20,
            size: square_size,
            color: 5,
            fill: true,
        };

        virtual_frame_buffer.draw_square(bkg_square);

        //Pixels
        for row in 0..SPRITE_SIZE.height {
            for column in 0..SPRITE_SIZE.width {
                let pos_x: usize = column * EDITOR_PIXEL_SIZE.width + column + bkg_square.pos_x + 2;
                let pos_y: usize = row * EDITOR_PIXEL_SIZE.height + row + bkg_square.pos_y + 2;

                let pixel_square: Square = Square {
                    pos_x,
                    pos_y,
                    size: EDITOR_PIXEL_SIZE,
                    color: 0,
                    fill: true,
                };

                virtual_frame_buffer.draw_square(pixel_square);

                //Highlight pixel if selected
                if self.selected_pixel_x == column && self.selected_pixel_y == row {
                    let highlight_square: Square = Square {
                        pos_x: pos_x - 1,
                        pos_y: pos_y - 1,
                        size: EDITOR_PIXEL_HIGHLIGHT_SIZE,
                        color: 7,
                        fill: false,
                    };

                    virtual_frame_buffer.draw_square(highlight_square);
                }
            }
        }
    }
}
