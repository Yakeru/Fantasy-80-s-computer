use app_macro::*;
use app_macro_derive::AppMacro;
use winit::event::{ElementState, KeyboardInput};

//use crate::text_layer::TextLayerChar;
use virtual_frame_buffer::*;
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
    selected_color: u8,
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
            selected_color: 7
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
            self.started = true;
        }

        match char_received {
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

        match keybord_input {
            Some(k) => {
                if k.state == ElementState::Pressed {
                    match k.virtual_keycode {
                        Some(code) => {
                            match code {
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

                                VirtualKeyCode::Space => {
                                    self.pixel_grid[self.selected_pixel_y * SPRITE_SIZE.width + self.selected_pixel_x] = self.selected_color;
                                }

                                VirtualKeyCode::Delete => {
                                    self.pixel_grid[self.selected_pixel_y * SPRITE_SIZE.width + self.selected_pixel_x] = 0;
                                }

                                VirtualKeyCode::PageUp => {}

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
            }
            None => (),
        }

        return response;
    }

    pub fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
        //virtual_frame_buffer.get_text_layer().clear();
        //virtual_frame_buffer.get_text_layer().show_cursor = false;

        //Drawing are Background square
        let bkg_square_width = SPRITE_SIZE.width * EDITOR_PIXEL_SIZE.width + SPRITE_SIZE.width + 3;
        let bkg_square_height =
            SPRITE_SIZE.height * EDITOR_PIXEL_SIZE.height + SPRITE_SIZE.height + 3;
        let square_size: PhysicalSize<usize> =
            PhysicalSize::new(bkg_square_width, bkg_square_height);

        let bkg_square: Square = Square {
            pos_x: 20,
            pos_y: 20,
            width: bkg_square_width,
            height: bkg_square_height,
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
                    width: EDITOR_PIXEL_SIZE.width,
                    height: EDITOR_PIXEL_SIZE.height,
                    color: self.pixel_grid[row * SPRITE_SIZE.width + column],
                    fill: true,
                };

                virtual_frame_buffer.draw_square(pixel_square);

                //Highlight pixel if selected
                if self.selected_pixel_x == column && self.selected_pixel_y == row {
                    let highlight_square: Square = Square {
                        pos_x: pos_x - 1,
                        pos_y: pos_y - 1,
                        width: EDITOR_PIXEL_HIGHLIGHT_SIZE.width,
                        height: EDITOR_PIXEL_HIGHLIGHT_SIZE.height,
                        color: 7,
                        fill: false,
                    };

                    virtual_frame_buffer.draw_square(highlight_square);
                }
            }
        }
    }
}
