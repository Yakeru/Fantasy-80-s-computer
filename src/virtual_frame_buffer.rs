use crate::{characters_rom::rom};
use crate::color_palettes::*;
use crate::sprite::Sprite;
use crate::text_layer::TextLayer;
use rand::Rng;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use crate::config;

const WIDTH: usize = config::WIDTH;
const HEIGHT: usize = config::HEIGHT;
const VIRTUAL_WIDTH: usize = config::VIRTUAL_WIDTH;
const VIRTUAL_HEIGHT: usize = config::VIRTUAL_HEIGHT;
const UPSCALE: usize = config::UPSCALE;
const SCAN_LINE_STRENGTH: u8 = config::SCAN_LINE_STRENGTH;

const SUB_PIXEL_COUNT: usize = 4;
const RENDERED_LINE_LENGTH: usize = WIDTH * SUB_PIXEL_COUNT;

/// Contains a list of u8 values corresponding to values from a color palette.
/// So just one u8 per pixel, R G and B values are retrieved from the palette, No Alpha.
/// This frame buffer is meant to contain a low resolution low color picure that
/// will be upscaled into the final pixel 2D frame buffer.
pub struct VirtualFrameBuffer {
    frame_time_ms: u64,
    frame: Box<[u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]>,
    color_palette: ColorPalette,
    line_scroll_list: [i8; VIRTUAL_HEIGHT],
    text_layer: TextLayer,
    sprites: Vec<Sprite>,
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
    pub size: PhysicalSize<usize>,
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
    pub fn new(frame_time_ms: u64) -> VirtualFrameBuffer {
        let virtual_frame_buffer: Box<[u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]> = Box::new([0; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]);
        let text_layer: TextLayer = TextLayer::new();
        let sprites: Vec<Sprite> = Vec::new();

        //TODO init background_layers, tiles_layers, sprites_layers... and correesponding renderes

        VirtualFrameBuffer {
            frame_time_ms,
            frame: virtual_frame_buffer,
            color_palette: ColorPalette::new(),
            line_scroll_list: [0; VIRTUAL_HEIGHT],
            text_layer,
            sprites,
            frame_counter: 0,
            second_tick: false,
            half_second_tick: false,
            half_second_latch: false,
        }
    }

    pub fn get_window_size(&mut self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    pub fn get_frame(&mut self) -> &mut Box<[u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]> {
        &mut self.frame
    }

    pub fn get_frame_static(&self) -> &Box<[u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]> {
        &self.frame
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        let index = VirtualFrameBuffer::coord_to_vec_index(x, y);
        self.frame[index]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        let index = VirtualFrameBuffer::coord_to_vec_index(x, y);
        self.frame[index] = color
    }

    pub fn coord_to_vec_index(x: usize, y: usize) -> usize {
        (y * VIRTUAL_WIDTH + x) % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT)
    }

    pub fn get_line_scroll_list(&mut self) -> &mut [i8; VIRTUAL_HEIGHT] {
        &mut self.line_scroll_list
    }

    pub fn set_line_scroll_list(&mut self, index: usize, value: i8) {
        if index < self.line_scroll_list.len() {
            self.line_scroll_list[index] = value;
        }
    }

    pub fn draw_line(&mut self, line: Line) {
        //self.set_pixel(line.start_x, line.start_y, line.color);
        //self.set_pixel(line.end_x, line.end_y, line.color);

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
            self.set_pixel(x0 as usize, y0 as usize, line.color);

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

    pub fn draw_square(&mut self, square: Square) {
        let start_offset: usize =
            VirtualFrameBuffer::coord_to_vec_index(square.pos_x, square.pos_y);

        for row in 0..square.size.width {
            for column in 0..square.size.height {
                if square.fill {
                    let offset = (start_offset + column + VIRTUAL_WIDTH * row)
                        % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);
                    self.frame[offset] = square.color;
                } else {
                    if row == 0
                        || row == square.size.width - 1
                        || column == 0
                        || column == square.size.height - 1
                    {
                        let offset = (start_offset + column + VIRTUAL_WIDTH * row)
                            % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);
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
        //let clear_frame: [u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT] = [color; VIRTUAL_WIDTH * VIRTUAL_HEIGHT];
        self.frame
            .copy_from_slice(&[color; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]);
    }

    pub fn get_text_layer(&mut self) -> &mut TextLayer {
        &mut self.text_layer
    }

    pub fn get_width(&self) -> usize {
        VIRTUAL_WIDTH
    }

    pub fn get_height(&self) -> usize {
        VIRTUAL_HEIGHT
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

        self.sprite_layer_renderer();
        self.text_layer_renderer();
        self.apply_line_scroll_effect();
    }

    /// Gets all the sprites listed in the sprite vector and renders them at the right place in the
    /// the virtual frame buffer
    fn sprite_layer_renderer(&mut self) {

        for sprite in self.sprites.chunks_exact_mut(1) {
            let mut pixel_count = 0;
            let mut sprite_line_count = 0;

            let global_offset =
                VirtualFrameBuffer::coord_to_vec_index(sprite[0].pos_x, sprite[0].pos_y);

            for pixel in &sprite[0].image {
                let virtual_fb_offset =
                    (global_offset + VIRTUAL_WIDTH * sprite_line_count + pixel_count)
                        % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);

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

        let horizontal_border: usize = (VIRTUAL_HEIGHT - self.text_layer.get_dimensions().1 * 8) / 2;
        let vertical_border: usize = (VIRTUAL_WIDTH - self.text_layer.get_dimensions().0 * 8) / 2;
        let mut x_pos = horizontal_border;
        let mut y_pos = vertical_border;
        let mut text_row_count = 0;
        let mut text_col_count = 0;

        for char_counter in 0..self.text_layer.get_size() {

            let text_layer_char = self.text_layer.get_char_map()[char_counter];
            let text_layer_color = self.text_layer.get_color_map()[char_counter];

            match text_layer_char {
                Some(char) => {

                    let mut char_color = self.text_layer.get_default_color();
                    let mut bck_color = self.text_layer.get_default_bkg_color();
                    let mut blink = false;
                    let mut swap = false;
                    let mut shadowed = false;

                    match text_layer_color {
                        Some(color) => {
                            char_color = ((color & 0x00F0) >> 4) as u8;
                            bck_color = (color & 0x000F) as u8;
                            swap = color & 0x0100 != 0;
                            blink = color & 0x0200 != 0;
                            shadowed = color & 0x0400 != 0;
                        }

                        None => ()
                    }

                    //Blink
                    if blink && self.half_second_latch {
                        swap = !swap;
                    }

                    //set color, swap or not
                    let text_color = if swap {
                        bck_color
                    } else {
                        char_color
                    };
                    let text_bkg_color = if swap {
                        char_color
                    } else {
                        bck_color
                    };

                    //Get char picture from  "character rom"
                    let pic = rom(&char);

                    //Draw picture pixel by pixel in frame buffer
                    for row_count in 0..8 {
                        let row = pic[row_count];
                        let mut mask: u8 = 128;

                        for col_count in 0..8 {
                            let virtual_frame_buffer_pos = x_pos
                                + col_count
                                + (y_pos + row_count) * VIRTUAL_WIDTH;

                            if shadowed {
                                let shadow_mask:u8 = if row_count % 2 == 0 {0b10101010} else {0b01010101};
                                match shadow_mask & mask {
                                    0 => self.frame[virtual_frame_buffer_pos] = 0,
                                    _ => match row & mask {
                                        0 => self.frame[virtual_frame_buffer_pos] = text_bkg_color,
                                        _ => self.frame[virtual_frame_buffer_pos] = text_color,
                                    }
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

                None => ()
            }

            //Move to next character coordinates
            text_col_count += 1;
            x_pos += 8;

            if text_col_count == self.text_layer.get_dimensions().0 {
                text_col_count = 0;
                text_row_count += 1;
                x_pos = horizontal_border;
                y_pos += 8;
            }

            if text_row_count == self.text_layer.get_dimensions().1 {
                text_col_count = 0;
                text_row_count = 0;
                x_pos = horizontal_border;
                y_pos = vertical_border;
            }
        }
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
}

pub struct CrtEffectRenderer {
    scan_line_strength: u8,
}

impl CrtEffectRenderer {
    pub fn new() -> CrtEffectRenderer {
        CrtEffectRenderer {
            scan_line_strength: SCAN_LINE_STRENGTH,
        }
    }

    pub fn render(
        &self,
        virtual_frame_buffer: &VirtualFrameBuffer,
        output_frame: &mut [u8],
    ) {
        let now = Instant::now();

        if UPSCALE == 6 {

            //let rendered_scanline: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];
            let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];
            let mut rendered_ramp_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];

            let mut line_count: usize = 0;

            for virt_line in virtual_frame_buffer
                .get_frame_static()
                .chunks_exact(VIRTUAL_WIDTH)
            {
                let mut rgb_before: (u8, u8, u8) = (0,0,0);

                for pixel_index in 0..VIRTUAL_WIDTH {

                    let rgb: (u8, u8, u8) = virtual_frame_buffer
                        .color_palette
                        .get_rgb_from_index(virt_line[pixel_index]);

                    let rgb_after = if pixel_index < VIRTUAL_WIDTH - 1 {
                        virtual_frame_buffer
                        .color_palette
                        .get_rgb_from_index(virt_line[pixel_index + 1])
                    } else {
                        (0,0,0)
                    };

                    let scanline_alpha = u8::MAX - SCAN_LINE_STRENGTH;

                    let r1 = if rgb.0 > rgb_before.0 {
                            rgb.0 - ((rgb.0 - rgb_before.0)/5)
                        } 
                        else if rgb.0 < rgb_before.0 {
                            rgb.0 + ((rgb_before.0 - rgb.0)/5)
                        } else {
                            rgb.0
                        };

                    let g1 = if rgb.1 > rgb_before.1 {
                            rgb.1 - ((rgb.1 - rgb_before.1)/5)
                        } 
                        else if rgb.1 < rgb_before.1 {
                            rgb.1 + ((rgb_before.1 - rgb.1)/5)
                        } else {
                            rgb.1
                        };

                    let b1 = if rgb.2 > rgb_before.2 {
                            rgb.2 - ((rgb.2 - rgb_before.2)/5)
                        } 
                        else if rgb.2 < rgb_before.2 {
                            rgb.2 + ((rgb_before.2 - rgb.2)/5)
                        } else {
                            rgb.2
                        };

                    let r2 = if rgb.0 > rgb_after.0 {
                            rgb.0 - ((rgb.0 - rgb_after.0)/5)
                        } else if rgb.0 < rgb_after.0 {
                            rgb.0 + ((rgb_after.0 - rgb.0)/5)
                        } else {
                            rgb.0
                        };

                    let g2 = if rgb.1 > rgb_after.1 {
                            rgb.1 - ((rgb.1 - rgb_after.1)/5)
                        } else if rgb.1 < rgb_after.1 {
                            rgb.1 + ((rgb_after.1 - rgb.1)/5)
                        } else {
                            rgb.1
                        };

                    let b2 = if rgb.2 > rgb_after.2 {
                            rgb.2 - ((rgb.2 - rgb_after.2)/5)
                        } else if rgb.2 < rgb_after.2 {
                            rgb.2 + ((rgb_after.2 - rgb.2)/5)
                        } else {
                            rgb.2
                        };

                    let r1_index = 0 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ar1_index = 3 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                    let g1_index = 5 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ag1_index = 7 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                    let b1_index = 10 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ab1_index = 11 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                    let r2_index = 12 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ar2_index = 15 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                    let g2_index = 17 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ag2_index = 19 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                    let b2_index = 22 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ab2_index = 23 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                        
                    // rendered_scanline[r1_index] = r1/2;
                    // rendered_scanline[ar1_index] = scanline_alpha;
                    // rendered_scanline[g1_index] = g1/2;
                    // rendered_scanline[ag1_index] = scanline_alpha;
                    // rendered_scanline[b1_index] = b1/2;
                    // rendered_scanline[ab1_index] = scanline_alpha;
                    // rendered_scanline[r2_index] = r2/2;
                    // rendered_scanline[ar2_index] = scanline_alpha;
                    // rendered_scanline[g2_index] = g2/2;
                    // rendered_scanline[ag2_index] = scanline_alpha;
                    // rendered_scanline[b2_index] = b2/2;
                    // rendered_scanline[ab2_index] = scanline_alpha;

                    //--------------------------------------------------------------------------------------

                    rendered_ramp_line[r1_index] = r1/2;
                    rendered_ramp_line[ar1_index] = u8::MAX;
                    rendered_ramp_line[g1_index] = g1/2;
                    rendered_ramp_line[ag1_index] = u8::MAX;
                    rendered_ramp_line[b1_index] = b1/2;
                    rendered_ramp_line[ab1_index] = u8::MAX;
                    rendered_ramp_line[r2_index] = r2/2;
                    rendered_ramp_line[ar2_index] = u8::MAX;
                    rendered_ramp_line[g2_index] = g2/2;
                    rendered_ramp_line[ag2_index] = u8::MAX;
                    rendered_ramp_line[b2_index] = b2/2;
                    rendered_ramp_line[ab2_index] = u8::MAX;   

                    //--------------------------------------------------------------------------------------

                    rendered_line[r1_index] = r1;
                    rendered_line[ar1_index] = u8::MAX;
                    rendered_line[g1_index] = g1;
                    rendered_line[ag1_index] = u8::MAX;
                    rendered_line[b1_index] = b1;
                    rendered_line[ab1_index] = u8::MAX;
                    rendered_line[r2_index] = r2;
                    rendered_line[ar2_index] = u8::MAX;
                    rendered_line[g2_index] = g2;
                    rendered_line[ag2_index] = u8::MAX;
                    rendered_line[b2_index] = b2;
                    rendered_line[ab2_index] = u8::MAX;

                    rgb_before = rgb;
                }

                let start = line_count * UPSCALE * RENDERED_LINE_LENGTH;
                // output_frame[start..start + RENDERED_LINE_LENGTH]
                //     .copy_from_slice(&rendered_scanline);
                output_frame[start + RENDERED_LINE_LENGTH..start + 2 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_ramp_line);
                output_frame[start + 2 * RENDERED_LINE_LENGTH..start + 3 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                output_frame[start + 3 * RENDERED_LINE_LENGTH..start + 4 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                output_frame[start + 4 * RENDERED_LINE_LENGTH..start + 5 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_ramp_line);
                // output_frame[start + 5 * RENDERED_LINE_LENGTH..start + 6 * RENDERED_LINE_LENGTH]
                //     .copy_from_slice(&rendered_scanline);

                line_count += 1;
            }
        } else {

            let mut rendered_scanline: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];
            let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];

            let mut line_count: usize = 0;

            for virt_line in virtual_frame_buffer
                .get_frame_static()
                .chunks_exact(VIRTUAL_WIDTH)
            {
                for pixel_index in 0..VIRTUAL_WIDTH {

                    let rgb: (u8, u8, u8) = virtual_frame_buffer
                        .color_palette
                        .get_rgb_from_index(virt_line[pixel_index]);

                    let scanline_alpha = u8::MAX - SCAN_LINE_STRENGTH;

                    let r_index = 0 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ar_index = 3 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                    let g_index = 5 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ag_index = 7 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                    let b_index = 10 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                    let ab_index = 11 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                        
                    rendered_scanline[r_index] = rgb.0;
                    rendered_scanline[ar_index] = scanline_alpha;
                    rendered_scanline[g_index] = rgb.1;
                    rendered_scanline[ag_index] = scanline_alpha;
                    rendered_scanline[b_index] = rgb.2;
                    rendered_scanline[ab_index] = scanline_alpha;

                    //--------------------------------------------------------------------------------------

                    rendered_line[r_index] = rgb.0;
                    rendered_line[ar_index] = u8::MAX;
                    rendered_line[g_index] = rgb.1;
                    rendered_line[ag_index] = u8::MAX;
                    rendered_line[b_index] = rgb.2;
                    rendered_line[ab_index] = u8::MAX;
                }

                let start = line_count * UPSCALE * RENDERED_LINE_LENGTH;
                output_frame[start..start + RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                output_frame[start + RENDERED_LINE_LENGTH..start + 2 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                output_frame[start + 2 * RENDERED_LINE_LENGTH..start + 3 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_scanline);

                line_count += 1;
            }
        }

        println!("crt: {} micros", now.elapsed().as_micros());
    }
}
