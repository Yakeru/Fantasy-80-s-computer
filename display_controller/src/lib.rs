use std::{ops::{Range, RangeBounds, Bound}, cmp::{min, max}};
use characters_rom::*;
use clock::Clock;
use color_palettes::*;
use config::*;
use console::Console;
use rand::Rng;
use sprite::Sprite;
use text_layer::{TextLayer, text_index_to_frame_coord, TextLayerChar, index_to_text_coord};

pub mod config;
pub mod characters_rom;
pub mod color_palettes;
pub mod sprite;
pub mod text_layer;
pub mod console;

const SUB_PIXEL_COUNT: usize = 4;
const RENDERED_LINE_LENGTH: usize = VIRTUAL_WIDTH * SUB_PIXEL_COUNT;
const ROUNDED_CORNER: [usize;10] = [10, 8, 6, 5, 4, 3, 2, 2, 1, 1];

/// Contains a list of u8 values corresponding to values from a color palette.
/// So just one u8 per pixel, R G and B values are retrieved from the palette, No Alpha.
/// This frame buffer is meant to contain a low resolution low color picure that
/// will be upscaled into the final pixel 2D frame buffer.
pub struct DisplayController {
    frame: Box<[u8]>,
    overscan: [u8; VIRTUAL_HEIGHT],
    brightness: u8,
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
            brightness: 255,
            text_layer: TextLayer::new(),
            console: Console::new(),
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

    pub fn get_console(&self) -> &Console {
        &self.console
    }

    pub fn get_frame(&self) -> &Box<[u8]> {
        &self.frame
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<u8> {
        let index = frame_coord_to_index(x as isize, y as isize);
        if index.is_some() {
            return Some(self.frame[index.unwrap()])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: u8) {
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

    pub fn set_brightness(&mut self, br: u8) {
        self.brightness = br;
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

    pub fn is_inside_rounded_corner(&self, x: usize, y: usize) -> bool {

        if y < ROUNDED_CORNER.len() 
            && (x < ROUNDED_CORNER[y] || x >= VIRTUAL_WIDTH - ROUNDED_CORNER[y]) {
            return true
        }

        if y >= VIRTUAL_HEIGHT - ROUNDED_CORNER.len() 
            && (x < ROUNDED_CORNER[VIRTUAL_HEIGHT - y - 1] || x >= VIRTUAL_WIDTH - ROUNDED_CORNER[VIRTUAL_HEIGHT - y - 1]) {
            return true
        }

        return false
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

    pub fn render(&mut self, output_frame: &mut [u8]) {
        self.clock.update();

        //Sprites
        self.sprite_layer_renderer();
        
        //Text layer
        self.text_layer_renderer();
        
        // //Console
        // if self.console.display {
        //     self.console_renderer();
        // }

        //Line offset
        self.apply_line_scroll_effect();

        //Overscan
        self.overscan_renderer();

        self.render_to_output_frame(output_frame);

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
        let mut console_char_index = 0;

        for char_counter in 0..self.text_layer.get_len() {
            let frame_coord = text_index_to_frame_coord(char_counter);
            let char_coord = index_to_text_coord(char_counter);

            //if text coord inside console, render console, else render text layer buffer
            if self.console.display 
            && char_coord.0 >= self.console.get_coordinates().0 
            && char_coord.0 < self.console.get_coordinates().0 + self.console.get_size().0
            && char_coord.1 >= self.console.get_coordinates().1 
            && char_coord.1 < self.console.get_coordinates().1 + self.console.get_size().1
            {
                let char = self.console.get_formatted_buffer()[console_char_index];
                self.text_layer_char_renderer(&char, frame_coord.0, frame_coord.1);
                console_char_index += 1;
            } else {
                let text_layer_char = self.text_layer.get_char_map()[char_counter];
                match text_layer_char {
                    Some(char_struct) => {
                        self.text_layer_char_renderer(&char_struct, frame_coord.0, frame_coord.1);
                    },
                    None => ()
                }
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

    pub fn render_to_output_frame(&self, output_frame: &mut [u8]) {

        let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];

        let mut frame_line_count: usize = 0;

        for frame_line in self.frame.chunks_exact(VIRTUAL_WIDTH) {

            for frame_pixel in 0..VIRTUAL_WIDTH {

                let mut rgb = unsafe { COLOR_PALETTE[(frame_line[frame_pixel]) as usize]};
                
                if self.is_inside_rounded_corner(frame_pixel, frame_line_count) {
                    rgb = (0, 0, 0) 
                };

                let screen_pixel_index = SUB_PIXEL_COUNT * frame_pixel;

                let r = rgb.0;
                let r_index = 0 + screen_pixel_index;

                let g = rgb.1;
                let g_index = 1 + screen_pixel_index;

                let b = rgb.2;
                let b_index = 2 + screen_pixel_index;

                let a = self.brightness;
                let a_index = 3 + screen_pixel_index;

                rendered_line[r_index] = r;
                rendered_line[g_index] = g;
                rendered_line[b_index] = b;
                rendered_line[a_index] = a;
            }

            let start = frame_line_count * RENDERED_LINE_LENGTH;
            output_frame[start..start + RENDERED_LINE_LENGTH].copy_from_slice(&rendered_line);
            frame_line_count += 1;
        }
    }
    
    pub fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: u8) {

        if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                self.set_pixel(x, y1, color);
            }
            return
        }

        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                self.set_pixel(x1, y, color);
            }
            return
        }

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
            self.set_pixel(x0, y0, color);
    
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
    
    pub fn vector(&mut self, x: isize, y: isize, l: isize, color: u8, a:f32) -> (isize, isize) {
    
        let x1 = x;
        let y1 = y;
    
        let x_move = a.cos() * l as f32;
        let y_move = a.sin() * l as f32;
    
        let x2: isize;
        
        if x_move < 0.0 {
            x2 = x1 - (-x_move).round() as isize;
        } else {
            x2 = x1 + x_move.round() as isize;
        }
    
        let y2: isize;
       
        if y_move < 0.0 {
            y2 = y1 - (-y_move).round() as isize;
        } else {
            y2 = y1 + y_move.round() as isize;
        }
    
        self.line(x1, y1, x2, y2, color);

        return (x2, y2)
    }
    
    pub fn square(&mut self, x: isize, y: isize, width: isize, height: isize, color: u8, fill_color: u8, fill: bool) {
        self.line(x, y, x + width - 1, y, color);
        self.line(x + width - 1, y, x + width - 1, y + height - 1, color);
        self.line(x + width - 1, y + height - 1, x, y + height - 1, color);
        self.line(x, y + height - 1, x, y, color);

        if fill {
            for y in (y + 1)..(y + height - 1) {
                self.line(x + 1, y, x + width - 2, y, fill_color);
            }
        }
    }

    fn draw_circle(&mut self,  xc: isize, yc: isize, x: isize, y: isize, color: u8, fill_color: u8, fill: bool) {
        self.set_pixel(xc + x, yc + y, color);
        self.set_pixel(xc - x, yc + y, color);
        self.set_pixel(xc + x, yc - y, color);
        self.set_pixel(xc - x, yc - y, color);
        self.set_pixel(xc + y, yc + x, color);
        self.set_pixel(xc - y, yc + x, color);
        self.set_pixel(xc + y, yc - x, color);
        self.set_pixel(xc - y, yc - x, color);

        if fill {
            self.line(xc - x, yc + y - 1 , xc + x, yc + y - 1, fill_color);
            self.line(xc - x, yc - y + 1, xc + x, yc - y + 1, fill_color);
            self.line(xc - y + 1, yc + x, xc + y - 1, yc + x, fill_color);
            self.line(xc - y + 1, yc - x, xc + y - 1, yc - x, fill_color);
        }
    }
    
    pub fn circle(&mut self, xc: isize, yc: isize, r: usize, color: u8, fill_color: u8, mut fill: bool) {

        let mut x: isize = 0;
        let mut y: isize = r as isize;
        let mut d: isize = 3 - 2 * r as isize;

        //Special case for r = 1
        if r == 1 {

            self.set_pixel(xc, yc + 1, color);
            self.set_pixel(xc, yc - 1, color);
            self.set_pixel(xc + 1, yc, color);
            self.set_pixel(xc - 1, yc, color);

            if fill {self.set_pixel(xc, yc, fill_color)}

            return
        }

        self.draw_circle(xc, yc, x, y, color, fill_color, fill);

        while y >= x {
            x += 1;

            if d > 0 {
                d = d + 4 * (x as isize - y as isize) + 10;
                y -= 1;
            } else {
                d = d + 4 * x as isize + 6;
            }

            self.draw_circle(xc, yc, x, y, color, fill_color, fill);
        }
    }

    pub fn genrate_random_garbage(&mut self) {
        let mut random = rand::thread_rng();
            
        let frame: u8 = random.gen_range(0..32);
        self.clear(frame);
        self.get_text_layer_mut().clear();
    
        let char_map = self.get_text_layer_mut().get_char_map_mut();
        for index in 0..char_map.len() {
            
            let mut color: u8 = random.gen_range(0..40);
            color = if color > 31 { 0 } else { color };
    
            let mut bkg_color: u8 = random.gen_range(0..40);
            bkg_color = if bkg_color > 31 { 0 } else { bkg_color };
            
            let mut char_index = random.gen_range(0..100);
            char_index = if char_index > characters_rom::CHAR_TABLE.len() - 1 { 0 } else { char_index };
            let c:char = characters_rom::CHAR_TABLE[char_index];
    
            let effect:u8 = random.gen_range(0..10);
            let swap: bool = if effect & 0b00000001 > 0 {true} else {false};
            let blink: bool = if effect & 0b00000010 > 0 {true} else {false};
            let shadowed: bool = if effect & 0b00000100 > 0 {true} else {false};
    
            let text_layer_char: TextLayerChar = TextLayerChar{c, color, bkg_color, swap, blink, shadowed};
            char_map[index] = Some(text_layer_char);
        }
    }

    pub fn draw_loading_overscan_artefacts(&mut self) {
        let mut random = rand::thread_rng();
        let mut rgb_color: u8 = random.gen_range(0..32);
        let mut line_count: usize = 0;
        let mut band_height: usize = random.gen_range(4..20);
    
        while line_count <= VIRTUAL_HEIGHT {
            let range_max = if line_count + band_height > VIRTUAL_HEIGHT {VIRTUAL_HEIGHT } else { line_count + band_height };
            self.set_overscan_color_range(rgb_color, line_count..range_max);
            line_count += band_height;
            rgb_color = random.gen_range(0..32);
            band_height = random.gen_range(4..20);
        }
    }
}

pub const fn frame_coord_to_index(x: isize, y: isize) -> Option<usize> {
    if x < 0 {
        return None
    }

    if x >= VIRTUAL_WIDTH as isize {
        return None
    }

    if y < 0 {
        return None
    }

    if y >= VIRTUAL_HEIGHT as isize {
        return None
    }

    Some(y as usize * VIRTUAL_WIDTH + x as usize)
}