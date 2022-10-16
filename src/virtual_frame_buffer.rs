use crate::characters_rom::rom;
use crate::color_palettes::*;
use crate::sprite::Sprite;
use crate::text_layer::TextLayer;
use rand::Rng;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;

//4K
// const WIDTH: usize = 2560;
// const HEIGHT: usize = 1920;
// const VIRTUAL_WIDTH: usize = 853;
// const VIRTUAL_HEIGHT: usize = 640;

//1080
const WIDTH: usize = 1280;
const HEIGHT: usize = 960;
const VIRTUAL_WIDTH: usize = 426; // 426*3 = 1278 draw one black line on each side of screen for perfectly centered *3 scale
const VIRTUAL_HEIGHT: usize = 320; // 320*3 = 960

const H_UPSCALE: usize = 3;
const V_UPSCALE: usize = 3;

const SCAN_LINE_STRENGTH: u8 = 25;
const SUB_PIXEL_ATTENUATION: u8 = 230;

const SUB_PIXEL_COUNT: usize = 4;
const RENDERED_LINE_LENGTH: usize = WIDTH * SUB_PIXEL_COUNT;

/// Contains a list of u8 values corresponding to values from a color palette.
/// So just one u8 per pixel, R G and B values are retrieved from the palette, No Alpha.
/// This frame buffer is meant to contain a low resolution low color picure that
/// will be upscaled into the final pixel 2D frame buffer.
pub struct VirtualFrameBuffer {
    frame_time_ms: u64,
    frame: [u8; VIRTUAL_WIDTH * VIRTUAL_HEIGHT],
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
        let virtual_frame_buffer = [0; VIRTUAL_WIDTH * VIRTUAL_HEIGHT];
        let text_layer = TextLayer::new();
        let sprites = Vec::new();

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

    pub fn get_frame(&mut self) -> &mut [u8] {
        &mut self.frame
    }

    pub fn get_frame_static(&self) -> &[u8] {
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

        self.text_layer_renderer();
        self.sprite_layer_renderer();
        self.apply_line_scroll_effect();
    }

    /// Gets all the sprites listed in the sprite vector and renders them at the right place in the
    /// the virtual frame buffer
    fn sprite_layer_renderer(&mut self) {
        //let now = Instant::now();

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

        //println!("sprite: {} micros", now.elapsed().as_micros());
    }

    fn text_layer_renderer(&mut self) {
        //let now = Instant::now();

        let horizontal_border: usize = (VIRTUAL_HEIGHT - self.text_layer.get_size().1 * 8) / 2;
        let vertical_border: usize = (VIRTUAL_WIDTH - self.text_layer.get_size().0 * 8) / 2;
        let mut x_pos = horizontal_border;
        let mut y_pos = vertical_border;
        let mut text_row_count = 0;
        let mut text_col_count = 0;

        for character in self.text_layer.get_characters() {
            match character {
                Some(text_mode_char) => {
                    let mut flipp = text_mode_char.flipp;

                    //Blink
                    if text_mode_char.blink && self.half_second_latch {
                        flipp = !flipp;
                    }

                    let text_color = if flipp {
                        text_mode_char.background_color
                    } else {
                        text_mode_char.color
                    };
                    let text_bkg_color = if flipp {
                        text_color
                    } else {
                        text_mode_char.background_color
                    };

                    let pic = rom(&text_mode_char.unicode);

                    for row_count in 0..8 {
                        let row = pic[row_count];
                        let row_in_binary = &format!("{:0>8b}", row);
                        let mut character_sprite_col_count = 0;

                        for c in row_in_binary.chars() {
                            let virtual_frame_buffer_pos = x_pos
                                + character_sprite_col_count
                                + (y_pos + row_count) * VIRTUAL_WIDTH;

                            match c {
                                '0' => self.frame[virtual_frame_buffer_pos] = text_bkg_color,
                                '1' => self.frame[virtual_frame_buffer_pos] = text_color,
                                _ => (),
                            }
                            character_sprite_col_count += 1;
                        }
                    }
                }

                None => (),
            }

            text_col_count += 1;
            x_pos += 8;

            if text_col_count == self.text_layer.get_size().0 {
                text_col_count = 0;
                text_row_count += 1;
                x_pos = horizontal_border;
                y_pos += 8;
            }

            if text_row_count == self.text_layer.get_size().1 {
                text_col_count = 0;
                text_row_count = 0;
                x_pos = horizontal_border;
                y_pos = vertical_border;
            }
        }

        //println!("text: {} micros", now.elapsed().as_micros());
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
    sub_pixel_attenuation: u8,
}

impl CrtEffectRenderer {
    pub fn new() -> CrtEffectRenderer {
        CrtEffectRenderer {
            scan_line_strength: SCAN_LINE_STRENGTH,
            sub_pixel_attenuation: SUB_PIXEL_ATTENUATION,
        }
    }

    pub fn render(
        &self,
        virtual_frame_buffer: &VirtualFrameBuffer,
        output_frame: &mut [u8],
        ctr_effect_on: bool,
    ) {
        //let now = Instant::now();

        let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = [254; RENDERED_LINE_LENGTH];
        let mut rendered_scanline: [u8; RENDERED_LINE_LENGTH] =
            [254 - self.scan_line_strength; RENDERED_LINE_LENGTH];
        let mut line_count: usize = 0;

        for virt_line in virtual_frame_buffer
            .get_frame_static()
            .chunks_exact(VIRTUAL_WIDTH)
        {
            let mut count = 0;

            for virt_pixel in virt_line {
                let rgb: (u8, u8, u8) = virtual_frame_buffer
                    .color_palette
                    .get_rgb_from_index(*virt_pixel);
                let mut attenuated_rgb: (u8, u8, u8) = rgb;
                if ctr_effect_on {
                    attenuated_rgb = (
                        rgb.0.checked_sub(self.sub_pixel_attenuation).unwrap_or(0),
                        rgb.1.checked_sub(self.sub_pixel_attenuation).unwrap_or(0),
                        rgb.2.checked_sub(self.sub_pixel_attenuation).unwrap_or(0),
                    );
                }

                rendered_line[0 + SUB_PIXEL_COUNT * H_UPSCALE * count] = rgb.0;
                rendered_line[1 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.1;
                rendered_line[2 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.2;

                rendered_line[4 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.0;
                rendered_line[5 + SUB_PIXEL_COUNT * H_UPSCALE * count] = rgb.1;
                rendered_line[6 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.2;

                rendered_line[8 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.0;
                rendered_line[9 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.1;
                rendered_line[10 + SUB_PIXEL_COUNT * H_UPSCALE * count] = rgb.2;

                rendered_scanline[0 + SUB_PIXEL_COUNT * H_UPSCALE * count] = rgb.0;
                rendered_scanline[1 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.1;
                rendered_scanline[2 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.2;

                rendered_scanline[4 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.0;
                rendered_scanline[5 + SUB_PIXEL_COUNT * H_UPSCALE * count] = rgb.1;
                rendered_scanline[6 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.2;

                rendered_scanline[8 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.0;
                rendered_scanline[9 + SUB_PIXEL_COUNT * H_UPSCALE * count] = attenuated_rgb.1;
                rendered_scanline[10 + SUB_PIXEL_COUNT * H_UPSCALE * count] = rgb.2;

                count += 1;
            }

            let start = line_count * 3 * RENDERED_LINE_LENGTH;
            output_frame[start..start + RENDERED_LINE_LENGTH].copy_from_slice(&rendered_line);
            output_frame[start + RENDERED_LINE_LENGTH..start + 2 * RENDERED_LINE_LENGTH]
                .copy_from_slice(&rendered_line);
            output_frame[start + 2 * RENDERED_LINE_LENGTH..start + 3 * RENDERED_LINE_LENGTH]
                .copy_from_slice(&rendered_scanline);

            line_count += 1;
        }

        //println!("crt: {} micros", now.elapsed().as_micros());
    }
}
