use color_palettes::*;
use config::*;
use fantasy_cpc_clock::Clock;
use rand::prelude::*;
use sprite::Sprite;
use std::{
    ops::{Bound, RangeBounds}, time::Instant
};

use crate::text_layer::text_layer::TextLayer;

pub mod shapes_2d;
pub mod sprite_layer;
pub mod color_palettes;
pub mod config;
pub mod sprite;
pub mod text_layer;
const SUB_PIXEL_COUNT: usize = 4;
const RENDERED_LINE_LENGTH: usize = VIRTUAL_WIDTH * SUB_PIXEL_COUNT;
const ROUNDED_CORNER: [usize; 10] = [10, 8, 6, 5, 4, 3, 2, 2, 1, 1];

/// Contains a list of u8 values corresponding to values from a color palette.
/// So just one u8 per pixel, R G and B values are retrieved from the palette, No Alpha.
/// This frame buffer is meant to contain a low resolution low color picure that
/// will be upscaled into the final pixel 2D frame buffer.
pub struct DisplayController {
    frame: Vec<u8>,
    overscan: [u8; VIRTUAL_HEIGHT],
    brightness: u8,
    line_scroll_list: [isize; VIRTUAL_HEIGHT],
    text_layer: TextLayer,
    sprites: Vec<Sprite>,
    clock: Clock,
}

impl Default for DisplayController {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayController {
    pub fn new() -> DisplayController {
        //TODO init background_layers, tiles_layers, sprites_layers... and correesponding renderes

        DisplayController {
            frame: vec![0; VIRTUAL_WIDTH * VIRTUAL_HEIGHT],
            overscan: [WHITE; VIRTUAL_HEIGHT],
            line_scroll_list: [0; VIRTUAL_HEIGHT],
            brightness: 255,
            text_layer: TextLayer::new(),
            sprites: Vec::new(),
            clock: Clock::new(),
        }
    }

    pub fn get_frame_size(&self) -> (usize, usize) {
        (VIRTUAL_WIDTH, VIRTUAL_HEIGHT)
    }

    pub fn get_text_layer_size_xy(&self) -> (usize, usize) {
        self.text_layer.get_dimensions_xy()
    }

    pub fn get_frame_mut(&mut self) -> &mut Vec<u8> {
        &mut self.frame
    }

    pub fn get_frame(&self) -> &Vec<u8> {
        &self.frame
    }

    pub fn get_pixel(&mut self, x: isize, y: isize) -> Option<u8> {
        let index = frame_coord_to_index(x, y);

        if let Some(i) = index {
            return Some(self.frame[i]);
        }

        None
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: u8) {
        let index = frame_coord_to_index(x, y);
        if let Some(i) = index {
            self.frame[i] = color;
        }
    }

    pub fn get_line_scroll_list(&mut self) -> &mut [isize] {
        &mut self.line_scroll_list
    }

    pub fn set_line_scroll_list(&mut self, index: usize, value: isize) {
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
        for (line_count, line) in self.frame.chunks_exact_mut(VIRTUAL_WIDTH).enumerate() {
            if !(OVERSCAN_V..VIRTUAL_HEIGHT - OVERSCAN_V).contains(&line_count) {
                line.copy_from_slice(&[self.overscan[line_count]; VIRTUAL_WIDTH]);
            } else {
                line.chunks_exact_mut(OVERSCAN_H)
                    .next()
                    .unwrap()
                    .copy_from_slice(&[self.overscan[line_count]; OVERSCAN_H]);
                line.chunks_exact_mut(OVERSCAN_H)
                    .last()
                    .unwrap()
                    .copy_from_slice(&[self.overscan[line_count]; OVERSCAN_H]);
            }
        }
    }

    pub fn rounded_corners_renderer(&mut self) {

        for (line_index, nb_of_black_pixels) in ROUNDED_CORNER.iter().enumerate() {
            
            let top_line = self.frame.chunks_exact_mut(VIRTUAL_WIDTH).nth(line_index);
            if let Some(line) = top_line {
                let mut chunk_iter = line.chunks_exact_mut(*nb_of_black_pixels);

                let first = chunk_iter.next();
                if let Some(c) = first {
                    c.fill(0);
                }

                let last = chunk_iter.last();
                if let Some(c) = last {
                    c.fill(0);
                }
            }

            let bottom_line = self.frame.chunks_exact_mut(VIRTUAL_WIDTH).nth_back(line_index);
            if let Some(line) = bottom_line {
                let mut chunk_iter = line.chunks_exact_mut(*nb_of_black_pixels);

                let first = chunk_iter.next();
                if let Some(c) = first {
                    c.fill(0);
                }

                let last = chunk_iter.last();
                if let Some(c) = last {
                    c.fill(0);
                }
            }
        }
    }

    pub fn is_inside_rounded_corner(&self, x: usize, y: usize) -> bool {
        if y < ROUNDED_CORNER.len()
            && (x < ROUNDED_CORNER[y] || x >= VIRTUAL_WIDTH - ROUNDED_CORNER[y])
        {
            return true;
        }

        if y >= VIRTUAL_HEIGHT - ROUNDED_CORNER.len()
            && (x < ROUNDED_CORNER[VIRTUAL_HEIGHT - y - 1]
                || x >= VIRTUAL_WIDTH - ROUNDED_CORNER[VIRTUAL_HEIGHT - y - 1])
        {
            return true;
        }

        false
    }

    /// Sets all the pixels to the specified color of the color palette
    /// Used to clear the screen between frames or set the background when
    /// redering only the text layer. Doesn't include the overscan.
    pub fn clear(&mut self, color: u8) {
        self.frame.fill(color);
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

        let start = Instant::now();

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

        //Rounded corners
        self.rounded_corners_renderer();

        self.render_to_output_frame(output_frame);

        self.clock.count_frame();

        dbg!(start.elapsed());
    }

    fn apply_line_scroll_effect(&mut self) {
        for (line_index, line_scroll_value) in self.line_scroll_list.into_iter().enumerate() {
            if line_scroll_value > 0 {
                self.frame[VIRTUAL_WIDTH * line_index..VIRTUAL_WIDTH * line_index + VIRTUAL_WIDTH]
                    .rotate_right(line_scroll_value as usize);
            }

            if line_scroll_value < 0 {
                self.frame[VIRTUAL_WIDTH * line_index..VIRTUAL_WIDTH * line_index + VIRTUAL_WIDTH]
                    .rotate_left((-line_scroll_value) as usize);
            }
        }
    }

    pub fn render_to_output_frame(&self, output_frame: &mut [u8]) {
        let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];

        for (frame_line_count, frame_line) in self.frame.chunks_exact(VIRTUAL_WIDTH).enumerate() {
            for frame_pixel in 0..VIRTUAL_WIDTH {

                let rgb = unsafe { COLOR_PALETTE[frame_line[frame_pixel as usize] as usize] };

                let screen_pixel_index = SUB_PIXEL_COUNT * frame_pixel;

                let r = rgb.0;
                let r_index = screen_pixel_index;

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
        }
    }

    pub fn draw_loading_overscan_artefacts(&mut self) {
        let mut random = rand::rng();
        let mut rgb_color: u8 = random.random_range(0..32);
        let mut line_count: usize = 0;
        let mut band_height: usize = random.random_range(4..20);

        while line_count <= VIRTUAL_HEIGHT {
            let range_max = if line_count + band_height > VIRTUAL_HEIGHT {
                VIRTUAL_HEIGHT
            } else {
                line_count + band_height
            };
            self.set_overscan_color_range(rgb_color, line_count..range_max);
            line_count += band_height;
            rgb_color = random.random_range(0..32);
            band_height = random.random_range(4..20);
        }
    }
}

pub const fn frame_coord_to_index(x: isize, y: isize) -> Option<usize> {
    if x < 0 {
        return None;
    }

    if x >= VIRTUAL_WIDTH as isize {
        return None;
    }

    if y < 0 {
        return None;
    }

    if y >= VIRTUAL_HEIGHT as isize {
        return None;
    }

    Some(y as usize * VIRTUAL_WIDTH + x as usize)
}
