use crate::color_palettes::*;
use crate::text_layer::TextLayer;
use crate::characters_rom::rom;
use crate::sprite::Sprite;
use std::time::{
    Instant, Duration
};
use winit::dpi::PhysicalSize;

/// Contains a list of u8 values corresponding to values from a color palette.
/// So just one u8 per pixel, R G and B values are retrieved from the palette, No Alpha.
/// This frame buffer is meant to contain a low resolution low color picure that 
/// will be upscaled into the final pixel 2D frame buffer.
pub struct VirtualFrameBuffer {
    frame_time_ms: u64,
    width: usize,
    height: usize,
    columns_count: usize,
    rows_count: usize,
    frame: Vec<u8>,
    text_layer: TextLayer,
    sprites: Vec<Sprite>,
    //background_layer
    //tiles_layer
    frame_counter: usize,
    second_tick: bool,
    half_second_tick: bool,
    half_second_latch: bool
}

#[derive(Copy, Clone)]
pub struct Square {
    pub pos_x: usize,
    pub pos_y: usize,
    pub size: PhysicalSize<usize>,
    pub color: u8,
    pub fill: bool
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
    pub fn new(frame_time_ms: u64, fb_width: usize, 
        fb_height: usize, 
        columns_count: usize, 
        rows_count: usize, 
        default_text_color: u8,
        default_text_bkg_color: u8) -> VirtualFrameBuffer {
        let size = fb_width * fb_height;
        let mut virtual_frame_buffer = Vec::new();

        for _value in 0..size {
            virtual_frame_buffer.push(0);
        }

        let text_layer = TextLayer::new(columns_count, rows_count);
        let sprites = Vec::new();
        
        //TODO init background_layers, tiles_layers, sprites_layers... and correesponding renderes

        VirtualFrameBuffer {
            frame_time_ms,
            width: fb_width,
            height: fb_height,
            columns_count,
            rows_count,
            frame: virtual_frame_buffer,
            text_layer,
            sprites,
            frame_counter: 0,
            second_tick: false,
            half_second_tick: false,
            half_second_latch: false
        }
    }

    pub fn get_frame(&mut self) -> &mut [u8] {
        &mut self.frame
    }

    pub fn get_frame_static(&self) -> &[u8] {
        &self.frame
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        let index = VirtualFrameBuffer::coord_to_vec_index(x, y, self.width, self.height);
        self.frame[index]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        let index = VirtualFrameBuffer::coord_to_vec_index(x, y, self.width, self.height);
        self.frame[index] = color
    }

    pub fn coord_to_vec_index(x: usize, y: usize, width: usize, height: usize) -> usize {
        (y * width + x) % (width * height)
    }

    pub fn draw_line(&mut self, line: Line) {
        //self.set_pixel(line.start_x, line.start_y, line.color);
        //self.set_pixel(line.end_x, line.end_y, line.color);

        let dx: isize = (line.end_x as isize - line.start_x as isize).abs();
        let dy: isize = -(line.end_y as isize - line.start_y as isize).abs();
        let sx: isize = if line.start_x < line.end_x {1} else {-1};
        let sy: isize = if line.start_y < line.end_y {1} else {-1};
        let mut error = dx + dy;

        let mut x0 = line.start_x as isize;
        let mut y0 = line.start_y as isize;
        let x1 = line.end_x as isize;
        let y1 = line.end_y as isize;

        while true {

            self.set_pixel(x0 as usize, y0 as usize, line.color);

            if x0 == x1 && y0 == y1 {break};
            let e2 = 2 * error;

            if e2 >= dy {
                if x0 == x1 {break};
                error += dy;
                x0 += sx;
            }

            if e2 <= dx {
                if y0 == y1 {break};
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_square(&mut self, square: Square) {

        let start_offset: usize = VirtualFrameBuffer::coord_to_vec_index(square.pos_x, square.pos_y, self.width, self.height);

        for row in 0..square.size.width {
            for column in 0..square.size.height {
                if square.fill {
                    let offset = (start_offset + column + self.width * row)  % (self.width * self.height);
                    self.frame[offset] = square.color;
                } else {
                    if row == 0 || row == square.size.width - 1 || column == 0 || column == square.size.height - 1 {
                        let offset = (start_offset + column + self.width * row) % (self.width * self.height);
                        self.frame[offset] = square.color;
                    }
                }
            }
        }
    }

    /// Sets all the pixels to the specified color of the color palette
    /// Used to clear the screen between frames or set the background when
    /// redering only the text layer
    pub fn clear_frame_buffer(&mut self, color: u8) {
        for value in self.frame.chunks_exact_mut(1) {
            value[0] = color;
        }
    }

    pub fn get_text_layer(&mut self) -> &mut TextLayer {
        &mut self.text_layer
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_sprites(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
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

        self.text_layer_renderer();
        self.sprite_layer_renderer();
        //Add background renderees, sprite renderers etc...
    }

    /// Gets all the sprites listed in the sprite vector and renders them at the right place in the
    /// pixel vector of the virtual frame buffer
    fn sprite_layer_renderer(&mut self) {

        for sprite in self.sprites.chunks_exact_mut(1) {

            let mut pixel_count = 0;
            let mut sprite_line_count = 0;

            let global_offset = VirtualFrameBuffer::coord_to_vec_index(sprite[0].pos_x, sprite[0].pos_y, self.width, self.height);

            for pixel in &sprite[0].image {
        
                let virtual_fb_offset = (global_offset + self.width * sprite_line_count + pixel_count) % (self.width * self.height);

                if *pixel != 0 {
                    self.frame[virtual_fb_offset] = *pixel;
                }
    
                pixel_count += 1;
                if pixel_count == sprite[0].value_in_physical_size().width {
                    pixel_count = 0;
                    sprite_line_count += 1;
                }
            }
        }
    }

    fn text_layer_renderer(&mut self) {
        let horizontal_border: usize = (self.width - self.columns_count * 8) / 2;
        let vertical_border: usize = (self.height - self.rows_count * 8) / 2;
        let mut x_pos = horizontal_border;
        let mut y_pos = vertical_border;
        let mut text_row_count = 0;
        let mut text_col_count = 0;
    
        for character in self.text_layer.get_characters() {

            match character {
                Some(text_mode_char) => {

                    let text_color = text_mode_char.color;
                    let text_bkg_color = text_mode_char.background_color;
                    let mut flipp = text_mode_char.flipp;

                    //Blink
                    if text_mode_char.blink && self.half_second_latch {
                        flipp = !flipp;
                    }

                    let pic = rom(&text_mode_char.unicode);

                    for row_count in 0..8 {
            
                        let row = pic[row_count];
                        let row_in_binary = &format!("{:0>8b}", row);
                        let mut character_sprite_col_count = 0;
            
                        for c in row_in_binary.chars() {
                            let virtual_frame_buffer_pos = x_pos + character_sprite_col_count + (y_pos + row_count ) * self.width;

                            match c {
                                '0' => self.frame[virtual_frame_buffer_pos] = if flipp {text_color} else {text_bkg_color},
                                '1' => self.frame[virtual_frame_buffer_pos] = if flipp {text_bkg_color} else {text_color},
                                _ => ()
                            }
                            character_sprite_col_count += 1;
                        }
                    }
                }

                None => ()
            }
            
            text_col_count += 1;
            x_pos += 8;
    
            if text_col_count == self.columns_count {
                text_col_count = 0;
                text_row_count += 1;
                x_pos = horizontal_border;
                y_pos += 8;
            } 
    
            if text_row_count == self.rows_count {
                text_col_count = 0;
                text_row_count = 0;
                x_pos = horizontal_border;
                y_pos = vertical_border;
            }
        }
    }
}

pub struct CrtEffectRenderer {
    output_frame_px_width: usize,
    output_frame_px_height: usize,
    output_nb_of_values_per_pixel: usize,
    render_horiz_upscale: usize,
    render_vert_upscale: usize,
    scan_line_strength: u8,
    sub_pixel_attenuation: u8,
}


impl CrtEffectRenderer {

    pub fn new(output_width: usize, output_height: usize) -> CrtEffectRenderer {
        CrtEffectRenderer {
            output_frame_px_width: output_width,
            output_frame_px_height: output_height,
            render_horiz_upscale: 3,
            render_vert_upscale: 3,
            output_nb_of_values_per_pixel: 4,
            scan_line_strength: 25, //35,
            sub_pixel_attenuation: 230,
        }
    }

    pub fn render(&self, virtual_frame_buffer: &VirtualFrameBuffer, output_frame: &mut[u8], ctr_effect_on: bool) {

        let start: Instant = Instant::now();

        let mut rendered_line: [u8; 2560 * 4] = [0; 2560 * 4];
        let mut rendered_scanline: [u8; 2560 * 4] = [0; 2560 * 4];

        let mut line_count: usize = 0;

        for line in virtual_frame_buffer.get_frame_static().chunks_exact(853) {

            for i in 0..852 {

                let rgb: (u8, u8, u8) = get_rgb(&line[i]);

                rendered_line[0 + 12 * i] = rgb.0;
                rendered_line[1 + 12 * i] = rgb.1.checked_sub(self.sub_pixel_attenuation).unwrap_or(0);
                rendered_line[2 + 12 * i] = rgb.2.checked_sub(self.sub_pixel_attenuation).unwrap_or(0);
                rendered_line[3 + 12 * i] = 254;

                rendered_line[4 + 12 * i] = rgb.0.checked_sub(self.sub_pixel_attenuation).unwrap_or(0);
                rendered_line[5 + 12 * i] = rgb.1;
                rendered_line[6 + 12 * i] = rgb.2.checked_sub(self.sub_pixel_attenuation).unwrap_or(0);
                rendered_line[7 + 12 * i] = 254;

                rendered_line[8 + 12 * i] = rgb.0.checked_sub(self.sub_pixel_attenuation).unwrap_or(0);
                rendered_line[9 + 12 * i] = rgb.1.checked_sub(self.sub_pixel_attenuation).unwrap_or(0);
                rendered_line[10 + 12 * i] = rgb.2;
                rendered_line[11 + 12 * i] = 254;

                rendered_scanline[0 + 12 * i] = rgb.0;
                rendered_scanline[1 + 12 * i] = rgb.1.checked_sub(self.scan_line_strength).unwrap_or(0);
                rendered_scanline[2 + 12 * i] = rgb.2.checked_sub(self.scan_line_strength).unwrap_or(0);
                rendered_scanline[3 + 12 * i] = 254;

                rendered_scanline[4 + 12 * i] = rgb.0.checked_sub(self.scan_line_strength).unwrap_or(0);
                rendered_scanline[5 + 12 * i] = rgb.1;
                rendered_scanline[6 + 12 * i] = rgb.2.checked_sub(self.scan_line_strength).unwrap_or(0);
                rendered_scanline[7 + 12 * i] = 254;

                rendered_scanline[8 + 12 * i] = rgb.0.checked_sub(self.scan_line_strength).unwrap_or(0);
                rendered_scanline[9 + 12 * i] = rgb.1.checked_sub(self.scan_line_strength).unwrap_or(0);
                rendered_scanline[10 + 12 * i] = rgb.2;
                rendered_scanline[11 + 12 * i] = 254;
            }

            let start = line_count * 2560 * 4 * 3;
            output_frame[start..start + 2560 * 4].copy_from_slice(&rendered_line);
            output_frame[start + 2560 * 4..start + 2 * 2560 * 4].copy_from_slice(&rendered_line);
            output_frame[start + 2 * 2560 * 4..start + 3 * 2560 * 4].copy_from_slice(&rendered_scanline);

            line_count += 1;
        }

        let end: Duration = Instant::now().duration_since(start);
        println!("Frame: {} ms", end.as_millis());
    }
}