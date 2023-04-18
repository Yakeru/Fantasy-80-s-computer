use std::ops::{Range, RangeBounds, Bound};
use characters_rom::*;
use clock::Clock;
use color_palettes::*;
use config::*;
use console::Console;
use sprite::Sprite;
use text_layer::{TextLayer, text_index_to_frame_coord, text_coord_to_frame_coord, TextLayerChar};

pub mod config;
pub mod characters_rom;
pub mod color_palettes;
pub mod sprite;
pub mod text_layer;
pub mod console;
pub mod renderer;

/// Contains a list of u8 values corresponding to values from a color palette.
/// So just one u8 per pixel, R G and B values are retrieved from the palette, No Alpha.
/// This frame buffer is meant to contain a low resolution low color picure that
/// will be upscaled into the final pixel 2D frame buffer.
pub struct DisplayController {
    frame: Box<[u8]>,
    overscan: [u8; VIRTUAL_HEIGHT],
    line_scroll_list: [i8; VIRTUAL_HEIGHT],
    text_layer: TextLayer,
    sprites: Vec<Sprite>,
    console: Console,
    //background_layer
    //tiles_layer
    clock: Clock
}

#[derive(Copy, Clone)]
pub struct Square {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub color: u8,
    pub fill: bool,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
    pub color: u8
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub x: usize,
    pub y: usize,
    pub r: usize,
    pub color: u8,
    pub fill: bool
}

impl DisplayController {
    pub fn new() -> DisplayController {

        //TODO init background_layers, tiles_layers, sprites_layers... and correesponding renderes

        DisplayController {
            frame: Box::new([0; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]),
            overscan: [WHITE; VIRTUAL_HEIGHT],
            line_scroll_list: [0; VIRTUAL_HEIGHT],
            text_layer: TextLayer::new(),
            console: Console::new(10, 10, 30, 10, 
                YELLOW, TRUE_BLUE, TextLayerChar { c: '\u{25AE}', color: YELLOW, bkg_color: TRUE_BLUE, 
                swap: false, blink: true, shadowed: false }, false, false),
            sprites: Vec::new(),
            clock: Clock::new()
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

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<u8> {
        let index = frame_coord_to_index(x, y);
        if index.is_some() {
            return Some(self.frame[index.unwrap()])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        let index = frame_coord_to_index(x, y);
        if index.is_some() {
            self.frame[index.unwrap()] = color
        }
    }

    pub fn get_line_scroll_list(&mut self) -> &mut [i8] {
        &mut self.line_scroll_list
    }

    pub fn set_line_scroll_list(&mut self, index: usize, value: i8) {
        if index < self.line_scroll_list.len() {
            self.line_scroll_list[index] = value;
        }
    }

    pub fn set_overscan_color(&mut self, color: u8) {
        self.set_overscan_color_range(color, 0..VIRTUAL_HEIGHT)
    }

    pub fn set_overscan_color_range<R: RangeBounds<usize>>(&mut self, color: u8, range: R) {

        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(&s) => s + 1,
            Bound::Included(&s) => s,
        };

        let end = match range.end_bound() {
            Bound::Unbounded => VIRTUAL_HEIGHT,
            Bound::Excluded(&t) => t.min(VIRTUAL_HEIGHT),
            Bound::Included(&t) => (t + 1).min(VIRTUAL_HEIGHT),
        };
        
        assert!(start <= end);
        
        for overscan_index in start..end {
            self.overscan[overscan_index] = color
        }
    }

    pub fn overscan_renderer(&mut self) {
        
        let mut line_count: usize = 0;
        
        for line in self.frame.chunks_exact_mut(VIRTUAL_WIDTH) {

            if line_count < OVERSCAN_V || line_count >= VIRTUAL_HEIGHT - OVERSCAN_V {
                line.copy_from_slice(&[self.overscan[line_count]; VIRTUAL_WIDTH]);
            } else {
                line.chunks_exact_mut(OVERSCAN_H).next().unwrap().copy_from_slice(&[self.overscan[line_count]; OVERSCAN_H]);
                line.chunks_exact_mut(OVERSCAN_H).last().unwrap().copy_from_slice(&[self.overscan[line_count]; OVERSCAN_H]);
            }

            line_count += 1;
        }
    }

    /// Sets all the pixels to the specified color of the color palette
    /// Used to clear the screen between frames or set the background when
    /// redering only the text layer. Doesn't include the overscan.
    pub fn clear(&mut self, color: u8) {
        //let clear_frame: [u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT] = [color; VIRTUAL_WIDTH * VIRTUAL_HEIGHT];
        self.frame
            .copy_from_slice(&[color; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]);
        self.overscan.copy_from_slice(&[color; VIRTUAL_HEIGHT]);
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
        self.clock.update();

        self.sprite_layer_renderer();
        self.text_layer_renderer();
        if self.console.display {
            self.console_renderer();
        }
        self.apply_line_scroll_effect();

        //Overscan
        self.overscan_renderer();

        self.clock.count_frame();
    }

    fn apply_line_scroll_effect(&mut self) {
        let mut line_index: usize = 0;
    
        for line_scroll_value in self.line_scroll_list {
            if line_scroll_value > 0 {
                self.frame[VIRTUAL_WIDTH * line_index..VIRTUAL_WIDTH * line_index + VIRTUAL_WIDTH]
                    .rotate_right(line_scroll_value as usize);
            }
    
            if line_scroll_value < 0 {
                self.frame[VIRTUAL_WIDTH * line_index..VIRTUAL_WIDTH * line_index + VIRTUAL_WIDTH]
                    .rotate_left((-line_scroll_value) as usize);
            }
    
            line_index += 1;
        }
    }
    
    /// Gets all the sprites listed in the sprite vector and renders them at the right place in the
    /// the virtual frame buffer
    fn sprite_layer_renderer(&mut self) {
        for sprite in &self.sprites {
            let mut pixel_count = 0;
            let mut sprite_line_count = 0;
    
            let global_offset = frame_coord_to_index(sprite.pos_x, sprite.pos_y);
    
            if global_offset.is_some() {
                for pixel in &sprite.image {
                    let virtual_fb_offset =
                        (global_offset.unwrap() + VIRTUAL_WIDTH * sprite_line_count + pixel_count)
                            % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);
        
                    if *pixel != 0 {
                        self.frame[virtual_fb_offset] = *pixel;
                    }
        
                    pixel_count += 1;
                    if pixel_count == sprite.size.size().0 as usize {
                        pixel_count = 0;
                        sprite_line_count += 1;
                    }
                }
            }
        }
    }
    
    fn text_layer_renderer(&mut self) {
        for char_counter in 0..self.text_layer.get_len() {
    
            let text_layer_char = self.text_layer.get_char_map()[char_counter];
            let frame_coord = text_index_to_frame_coord(char_counter);
    
            match text_layer_char {
    
                Some(char_struct) => {
                    self.text_layer_char_renderer(&char_struct, frame_coord.0, frame_coord.1);
                },
                None => ()
            }
        }
    }
    
    
    pub fn console_renderer(&mut self) {
        let empty_text_cell = TextLayerChar {
            c: ' ', color: self.console.default_color, bkg_color: self.console.default_bkg_color, 
            swap: false, blink: false, shadowed: false
        };
        let cursor = self.console.cursor;
        
        let mut char_index = 0;
        for row_count in 0..self.console.get_row_count() {
            let char_y = self.console.pos_y + row_count;        
            for col_count in 0..self.console.get_col_count() {
                let char_x = self.console.pos_x + col_count;
                let frame_coord = text_coord_to_frame_coord(char_x, char_y);
                let char = self.console.get_content().get(char_index).copied();
                if char.is_some() {
                    match char.unwrap() {
                        Some(char) => {
                            self.text_layer_char_renderer(&char, frame_coord.0, frame_coord.1);
                        },
                        None => {
                            self.text_layer_char_renderer(&empty_text_cell, frame_coord.0, frame_coord.1);
                        }
                    }
                } else {
                    self.text_layer_char_renderer(&empty_text_cell, frame_coord.0, frame_coord.1);
                }
    
                if char_index == self.console.get_content().len() {
                    self.text_layer_char_renderer(&cursor, frame_coord.0, frame_coord.1);
                }
    
                char_index += 1;
            }
        }
    }
    
    fn text_layer_char_renderer(&mut self, text_layer_char: &TextLayerChar, frame_x_pos: usize, frame_y_pos: usize) {
        let char = text_layer_char.c;
        let char_color = text_layer_char.color;
        let bck_color = text_layer_char.bkg_color;
        let blink = text_layer_char.blink;
        let swap = text_layer_char.swap;
        let shadowed = text_layer_char.shadowed;
    
        //set color, swap or not
        let text_color = if swap || (blink && self.clock.half_second_latch) { bck_color } else { char_color };
        let text_bkg_color = if swap || (blink && self.clock.half_second_latch) { char_color } else { bck_color };
    
        //Get char picture from  "character rom"
        let pic = rom(char);
    
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
                        0 => self.frame[virtual_frame_buffer_pos] = 0,
                        _ => match row & mask {
                            0 => self.frame[virtual_frame_buffer_pos] = text_bkg_color,
                            _ => self.frame[virtual_frame_buffer_pos] = text_color,
                        },
                    }
                } else {
                    match row & mask {
                        0 => self.frame[virtual_frame_buffer_pos] = text_bkg_color,
                        _ => self.frame[virtual_frame_buffer_pos] = text_color,
                    }
                }
    
                mask = mask >> 1;
            }
        }
    }
    
    pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u8) {
        let dx: isize = (x2 as isize - x1 as isize).abs();
        let dy: isize = -(y2 as isize - y1 as isize).abs();
        let sx: isize = if x1 < x2 { 1 } else { -1 };
        let sy: isize = if y1 < y2 { 1 } else { -1 };
        let mut error = dx + dy;
    
        let mut x0 = x1 as isize;
        let mut y0 = y1 as isize;
        let x1 = x2 as isize;
        let y1 = y2 as isize;
    
        loop {
            let index = frame_coord_to_index(x0 as usize, y0 as usize);
            if index.is_some() {
                self.frame[index.unwrap()] = color;
            }
    
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
    
    pub fn vector(&mut self, x: usize, y: usize, l: usize, color: u8, a:f32) -> (usize, usize) {
    
        let x1 = x;
        let y1 = y;
    
        let x_move = a.cos() * l as f32;
        let y_move = a.sin() * l as f32;
    
        let x2: usize;
        
        if x_move < 0.0 {
            x2 = x1 - (-x_move).round() as usize;
        } else {
            x2 = x1 + x_move.round() as usize;
        }
    
        let y2: usize;
       
        if y_move < 0.0 {
            y2 = y1 - (-y_move).round() as usize;
        } else {
            y2 = y1 + y_move.round() as usize;
        }
    
        self.line(x1, y1, x2, y2, color);

        return (x2, y2)
    }
    
    pub fn square(&mut self, x: usize, y: usize, width: usize, height: usize, color: u8, fill: bool) {
        let mut current_line: usize = 0;
        let line_range: Range<usize> = y..(y + height + 1);
    
        for virtual_frame_row in self.frame.chunks_exact_mut(VIRTUAL_WIDTH) { // TODO use advance_by once it's stable
            if line_range.contains(&current_line) {
                if current_line == y || current_line == y + height {
                    for pixel_index in x..(x + width + 1) {
                        if pixel_index < VIRTUAL_WIDTH {
                            virtual_frame_row[pixel_index] = color;
                        }
                    }
                } else {
                    if fill {
                        for pixel_index in x..(x + width + 1) {
                            if pixel_index < VIRTUAL_WIDTH {
                                virtual_frame_row[pixel_index] = color;
                            }
                        }
                    } else {
                        if x < VIRTUAL_WIDTH {
                            virtual_frame_row[x] = color;
                        }
    
                        if x + width < VIRTUAL_WIDTH {
                            virtual_frame_row[x + width] = color;
                        }
                    }
                }
            }
            current_line += 1;
            if current_line == y + height + 1 { break };
        }
    }
    
    pub fn circle(&mut self, x: usize, y: usize, r: usize, color: u8, fill: bool) {
    
        for b in 0..(r * 3/4  + 1) {
            let a: f64 = (((r * r) - (b * b)) as f64).sqrt();
    
            let point1 = (x + b, y + a.round() as usize);
            let point2 = (x + b, y - a.round() as usize);
            let point3 = (x - b, y + a.round() as usize);
            let point4 = (x - b, y - a.round() as usize);
    
            if !fill {
                let point = frame_coord_to_index(point1.0, point1.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
        
                let point = frame_coord_to_index(point2.0, point2.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
        
                let point = frame_coord_to_index(point3.0, point4.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
        
                let point = frame_coord_to_index(point4.0, point4.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
            } else {
                self.line(point1.0, point1.1, point3.0, point3.1, color);
                self.line(point2.0, point2.1, point4.0, point4.1, color);
            }
        }
    
        for a in 0..(r * 3/4 + 1) {
            let b: f64 = (((r * r) - (a * a)) as f64).sqrt();
    
            let point1 = (x + b.round() as usize, y + a);
            let point2 = (x + b.round() as usize, y - a);
            let point3 = (x - b.round() as usize, y + a);
            let point4 = (x - b.round() as usize, y - a);
            
            if !fill {
                let point = frame_coord_to_index(point1.0, point1.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
    
                let point = frame_coord_to_index(point2.0, point2.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
    
                let point = frame_coord_to_index(point3.0, point4.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
    
                let point = frame_coord_to_index(point4.0, point4.1);
                if point.is_some() {self.frame[point.unwrap()] = color};
            } else {
                self.line(point1.0, point1.1, point3.0, point3.1, color);
                self.line(point2.0, point2.1, point4.0, point4.1, color);
            }
        }
    }
}

pub const fn frame_coord_to_index(x: usize, y: usize) -> Option<usize> {
    if x < VIRTUAL_WIDTH && y < VIRTUAL_HEIGHT {
        Some(y * VIRTUAL_WIDTH + x)
    } else {
        None
    }
}