use characters_rom::{rom, CHARACTER_WIDTH, CHARACTER_HEIGHT};
use color_palettes::*;
use config::*;
use console::Console;
use sprite::Sprite;
use text_layer::{TextLayer, text_index_to_frame_coord, text_coord_to_frame_coord};
use text_layer_char::TextLayerChar;

pub mod config;
pub mod characters_rom;
pub mod text_layer_char;
pub mod color_palettes;
pub mod sprite;
pub mod text_layer;
pub mod console;
pub mod crt_renderer;

/// Contains a list of u8 values corresponding to values from a color palette.
/// So just one u8 per pixel, R G and B values are retrieved from the palette, No Alpha.
/// This frame buffer is meant to contain a low resolution low color picure that
/// will be upscaled into the final pixel 2D frame buffer.
pub struct VirtualFrameBuffer {
    frame_time_ms: u128,
    frame: Box<[u8]>,
    color_palette: ColorPalette,
    line_scroll_list: Box<[i8]>,
    text_layer: TextLayer,
    sprites: Vec<Sprite>,
    console: Console,
    //background_layer
    //tiles_layer
    frame_counter: usize,
    second_tick: bool,
    half_second_tick: bool,
    half_second_latch: bool,
}

#[derive(Copy, Clone)]
pub struct Square {
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,
    pub color: u8,
    pub fill: bool,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub color: u8,
}

impl VirtualFrameBuffer {
    pub fn new(frame_time_ms: u128) -> VirtualFrameBuffer {
        let virtual_frame_buffer: Box<[u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]> =
            Box::new([0; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]);
        let text_layer: TextLayer = TextLayer::new();
        let sprites: Vec<Sprite> = Vec::new();

        //TODO init background_layers, tiles_layers, sprites_layers... and correesponding renderes

        VirtualFrameBuffer {
            frame_time_ms,
            frame: virtual_frame_buffer,
            color_palette: ColorPalette::new(),
            line_scroll_list: Box::new([0; VIRTUAL_HEIGHT]),
            text_layer,
            console: Console::new(10, 10, 30, 10, 
                YELLOW, TRUEBLUE, TextLayerChar { c: '\u{25AE}', color: YELLOW, bkg_color: TRUEBLUE, 
                swap: false, blink: true, shadowed: false }, false, false),
            sprites,
            frame_counter: 0,
            second_tick: false,
            half_second_tick: false,
            half_second_latch: false,
        }
    }

    pub fn get_window_size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    pub fn get_virtual_fb_size(&self) -> (usize, usize) {
        (VIRTUAL_WIDTH, VIRTUAL_HEIGHT)
    }

    pub fn get_text_layer_size_xy(&self) -> (usize, usize) {
        self.text_layer.get_dimensions_xy()
    }

    pub fn get_frame_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.frame
    }

    pub fn get_console_mut(&mut self) -> &mut Console {
        &mut self.console
    }

    pub fn get_frame(&self) -> &Box<[u8]> {
        &self.frame
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        let index = frame_coord_to_index(x, y);
        self.frame[index]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        let index = frame_coord_to_index(x, y);
        self.frame[index] = color
    }

    pub fn get_line_scroll_list(&mut self) -> &mut Box<[i8]> {
        &mut self.line_scroll_list
    }

    pub fn set_line_scroll_list(&mut self, index: usize, value: i8) {
        if index < self.line_scroll_list.len() {
            self.line_scroll_list[index] = value;
        }
    }

    /// Sets all the pixels to the specified color of the color palette
    /// Used to clear the screen between frames or set the background when
    /// redering only the text layer
    pub fn clear_frame_buffer(&mut self, color: u8) {
        //let clear_frame: [u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT] = [color; VIRTUAL_WIDTH * VIRTUAL_HEIGHT];
        self.frame
            .copy_from_slice(&[color; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]);
    }

    //Removes all chars, colors and effects from the text_layer
    pub fn clear_text_layer(&mut self) {
        self.text_layer.clear();
    }

    pub fn get_text_layer_mut(&mut self) -> &mut TextLayer {
        &mut self.text_layer
    }

    pub fn get_text_layer(&self) -> &TextLayer {
        &self.text_layer
    }

    pub fn get_width(&self) -> usize {
        VIRTUAL_WIDTH
    }

    pub fn get_height(&self) -> usize {
        VIRTUAL_HEIGHT
    }

    pub fn get_sprites_mut(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }

    pub fn get_sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }

    pub fn render(&mut self) {
        self.second_tick = false;
        self.half_second_tick = false;

        if self.frame_counter == (1000 / self.frame_time_ms as usize) / 2 - 1 {
            self.half_second_tick = true;
            self.half_second_latch = !self.half_second_latch;
        }

        if self.frame_counter == (1000 / self.frame_time_ms as usize) - 1 {
            self.frame_counter = 0;
            self.second_tick = true;
            self.half_second_tick = true;
            self.half_second_latch = !self.half_second_latch;
        } else {
            self.frame_counter += 1;
        }

        sprite_layer_renderer(&self.sprites, &mut self.frame);
        text_layer_renderer(&self.text_layer, &mut self.frame, self.half_second_latch);
        if self.console.display {
            console_renderer(&self.console, &mut self.frame, self.half_second_latch);
        }
        apply_line_scroll_effect(&self.line_scroll_list, &mut self.frame);
    }
}

pub const fn frame_coord_to_index(x: usize, y: usize) -> usize {
    let safe_x = x % VIRTUAL_WIDTH;
    let safe_y = y % VIRTUAL_HEIGHT;
    safe_y * VIRTUAL_WIDTH + safe_x
}

fn apply_line_scroll_effect(line_scroll_list: &[i8], frame: &mut [u8]) {
    let mut line_index: usize = 0;

    for line_scroll_value in line_scroll_list {
        if *line_scroll_value > 0 {
            frame[VIRTUAL_WIDTH * line_index..VIRTUAL_WIDTH * line_index + VIRTUAL_WIDTH]
                .rotate_right(*line_scroll_value as usize);
        }

        if *line_scroll_value < 0 {
            frame[VIRTUAL_WIDTH * line_index..VIRTUAL_WIDTH * line_index + VIRTUAL_WIDTH]
                .rotate_left((-*line_scroll_value) as usize);
        }

        line_index += 1;
    }
}

/// Gets all the sprites listed in the sprite vector and renders them at the right place in the
/// the virtual frame buffer
fn sprite_layer_renderer(sprites: &Vec<Sprite>, frame: &mut [u8]) {
    for sprite in sprites {
        let mut pixel_count = 0;
        let mut sprite_line_count = 0;

        let global_offset = frame_coord_to_index(sprite.pos_x, sprite.pos_y);

        for pixel in &sprite.image {
            let virtual_fb_offset =
                (global_offset + VIRTUAL_WIDTH * sprite_line_count + pixel_count)
                    % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);

            if *pixel != 0 {
                frame[virtual_fb_offset] = *pixel;
            }

            pixel_count += 1;
            if pixel_count == sprite.value_in_physical_size().0 {
                pixel_count = 0;
                sprite_line_count += 1;
            }
        }
    }
}

fn text_layer_renderer(text_layer: &TextLayer, frame: &mut [u8], half_second_latch: bool) {
    for char_counter in 0..text_layer.get_len() {

        let text_layer_char = text_layer.get_char_map()[char_counter];
        let frame_coord = text_index_to_frame_coord(char_counter);

        match text_layer_char {

            Some(char_struct) => {
                text_layer_char_renderer(&char_struct, frame_coord.0, frame_coord.1, frame, half_second_latch);
            },
            None => ()
        }
    }
}


pub fn console_renderer(console: &Console, frame: &mut [u8], half_second_latch: bool) {
    let empty_text_cell = TextLayerChar {
        c: ' ', color: console.default_color, bkg_color: console.default_bkg_color, 
        swap: false, blink: false, shadowed: false
    };
    
    let mut char_index = 0;
    for row_count in 0..console.get_row_count() {
        let char_y = console.pos_y + row_count;        
        for col_count in 0..console.get_col_count() {
            let char_x = console.pos_x + col_count;
            let frame_coord = text_coord_to_frame_coord(char_x, char_y);
    
            let char = console.get_content().get(char_index);
            if char.is_some() {
                match char.unwrap() {
                    Some(char) => {
                        text_layer_char_renderer(&char, frame_coord.0, frame_coord.1, frame, half_second_latch);
                    },
                    None => {
                        text_layer_char_renderer(&empty_text_cell, frame_coord.0, frame_coord.1, frame, half_second_latch);
                    }
                }
            } else {
                text_layer_char_renderer(&empty_text_cell, frame_coord.0, frame_coord.1, frame, half_second_latch);
            }

            if char_index == console.get_content().len() {
                text_layer_char_renderer(&console.cursor, frame_coord.0, frame_coord.1, frame, half_second_latch);
            }

            char_index += 1;
        }
    }
}

fn text_layer_char_renderer(text_layer_char: &TextLayerChar, frame_x_pos: usize, frame_y_pos: usize, frame: &mut [u8], half_second_latch: bool) {
    let char = text_layer_char.c;
    let char_color = text_layer_char.color;
    let bck_color = text_layer_char.bkg_color;
    let blink = text_layer_char.blink;
    let swap = text_layer_char.swap;
    let shadowed = text_layer_char.shadowed;

    //set color, swap or not
    let text_color = if swap || (blink && half_second_latch) { bck_color } else { char_color };
    let text_bkg_color = if swap || (blink && half_second_latch) { char_color } else { bck_color };

    //Get char picture from  "character rom"
    let pic = rom(&char);

    //Draw picture pixel by pixel in frame buffer
    for row_count in 0..CHARACTER_HEIGHT {
        let row = pic[row_count];
        let mut mask: u8 = 128;

        for col_count in 0..CHARACTER_WIDTH {
            let virtual_frame_buffer_pos =
            frame_x_pos + col_count + (frame_y_pos + row_count) * VIRTUAL_WIDTH;

            if shadowed {
                let shadow_mask: u8 = if row_count % 2 == 0 {
                    0b10101010
                } else {
                    0b01010101
                };
                match shadow_mask & mask {
                    0 => frame[virtual_frame_buffer_pos] = 0,
                    _ => match row & mask {
                        0 => frame[virtual_frame_buffer_pos] = text_bkg_color,
                        _ => frame[virtual_frame_buffer_pos] = text_color,
                    },
                }
            } else {
                match row & mask {
                    0 => frame[virtual_frame_buffer_pos] = text_bkg_color,
                    _ => frame[virtual_frame_buffer_pos] = text_color,
                }
            }

            mask = mask >> 1;
        }
    }

}

pub fn draw_line(line: Line, frame: &mut [u8]) {
    let dx: isize = (line.end_x as isize - line.start_x as isize).abs();
    let dy: isize = -(line.end_y as isize - line.start_y as isize).abs();
    let sx: isize = if line.start_x < line.end_x { 1 } else { -1 };
    let sy: isize = if line.start_y < line.end_y { 1 } else { -1 };
    let mut error = dx + dy;

    let mut x0 = line.start_x as isize;
    let mut y0 = line.start_y as isize;
    let x1 = line.end_x as isize;
    let y1 = line.end_y as isize;

    loop {
        frame[frame_coord_to_index(x0 as usize, y0 as usize)] = line.color;

        if x0 == x1 && y0 == y1 {
            break;
        };
        let e2 = 2 * error;

        if e2 >= dy {
            if x0 == x1 {
                break;
            };
            error += dy;
            x0 += sx;
        }

        if e2 <= dx {
            if y0 == y1 {
                break;
            };
            error += dx;
            y0 += sy;
        }
    }
}

pub fn draw_square(square: Square, frame: &mut [u8]) {
    let start_offset: usize = frame_coord_to_index(square.pos_x, square.pos_y);

    for row in 0..square.width {
        for column in 0..square.height {
            if square.fill {
                let offset = (start_offset + column + VIRTUAL_WIDTH * row)
                    % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);
                frame[offset] = square.color;
            } else {
                if row == 0
                    || row == square.width - 1
                    || column == 0
                    || column == square.height - 1
                {
                    let offset = (start_offset + column + VIRTUAL_WIDTH * row)
                        % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);
                    frame[offset] = square.color;
                }
            }
        }
    }
}