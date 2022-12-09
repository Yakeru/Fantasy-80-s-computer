use app_macro::*;
use app_macro_derive::AppMacro;
use winit::event::ElementState;
use virtual_frame_buffer::*;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;

const DEFAULT_BKG_COLOR: u8 = 0;
const SPRITE_SIZE: PhysicalSize<usize> = PhysicalSize::new(16, 16);
const EDITOR_PIXEL_SIZE: PhysicalSize<usize> = PhysicalSize::new(10, 10);
const EDITOR_PIXEL_HIGHLIGHT_SIZE: PhysicalSize<usize> =
    PhysicalSize::new(EDITOR_PIXEL_SIZE.width + 2, EDITOR_PIXEL_SIZE.height + 2);

#[derive(AppMacro)]
pub struct SpriteEditor {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    pixel_grid: [u8; SPRITE_SIZE.width * SPRITE_SIZE.height],
    selected_pixel_x: usize,
    selected_pixel_y: usize,
    selected_color: u8,
    initialized: bool
}

impl SpriteEditor {
    pub fn new() -> SpriteEditor {
        SpriteEditor {
            enable_auto_escape: true,
            name: String::from("spriteEdit"),
            updating: false,
            drawing: false,
            initialized: false,
            pixel_grid: [0; SPRITE_SIZE.width * SPRITE_SIZE.height],
            selected_pixel_x: 0,
            selected_pixel_y: 0,
            selected_color: 7
        }
    }

    pub fn init_app(&mut self, _virtual_frame_buffer: &mut VirtualFrameBuffer) {}

    pub fn update_app(
        &mut self,
        app_message: AppMessage,
        _virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        let response = AppResponse::new();

        match app_message.char_received {
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

        match app_message.keyboard_input {
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

                                _ => (),
                            }
                        }

                        None => (),
                    }
                }
            }
            None => (),
        }

        return Some(response);
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_console_mut().display = false;
        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
        virtual_frame_buffer.get_text_layer_mut().clear();
        //virtual_frame_buffer.get_text_layer().show_cursor = false;

        //Drawing are Background square
        let bkg_square_width = SPRITE_SIZE.width * EDITOR_PIXEL_SIZE.width + SPRITE_SIZE.width + 3;
        let bkg_square_height =
            SPRITE_SIZE.height * EDITOR_PIXEL_SIZE.height + SPRITE_SIZE.height + 3;
        
        let bkg_square: Square = Square {
            x: 20,
            y: 20,
            width: bkg_square_width,
            height: bkg_square_height,
            color: 5,
            fill: true,
        };

        draw_square(bkg_square, virtual_frame_buffer.get_frame_mut());

        //Pixels
        for row in 0..SPRITE_SIZE.height {
            for column in 0..SPRITE_SIZE.width {
                let x: usize = column * EDITOR_PIXEL_SIZE.width + column + bkg_square.x + 2;
                let y: usize = row * EDITOR_PIXEL_SIZE.height + row + bkg_square.y + 2;

                let pixel_square: Square = Square {
                    x,
                    y,
                    width: EDITOR_PIXEL_SIZE.width,
                    height: EDITOR_PIXEL_SIZE.height,
                    color: self.pixel_grid[row * SPRITE_SIZE.width + column],
                    fill: true,
                };

                draw_square(pixel_square, virtual_frame_buffer.get_frame_mut());

                //Highlight pixel if selected
                if self.selected_pixel_x == column && self.selected_pixel_y == row {
                    let highlight_square: Square = Square {
                        x: x - 1,
                        y: y - 1,
                        width: EDITOR_PIXEL_HIGHLIGHT_SIZE.width,
                        height: EDITOR_PIXEL_HIGHLIGHT_SIZE.height,
                        color: 7,
                        fill: false,
                    };

                    draw_square(highlight_square, virtual_frame_buffer.get_frame_mut());
                }
            }
        }
    }
}
