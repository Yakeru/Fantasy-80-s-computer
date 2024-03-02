use characters_rom::*;
use color_palettes::*;
use config::*;
use fantasy_cpc_clock::Clock;
use sprite::Sprite;
use std::{
    cmp::{max, min},
    ops::{Bound, RangeBounds},
};
use text_layer::{
    text_coord_to_frame_coord, TextLayer
};

pub mod characters_rom;
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
    pub frame: Box<[usize]>,
    pub overscan: [usize; VIRTUAL_HEIGHT],
    pub brightness: u8,
    pub line_scroll_list: [isize; VIRTUAL_HEIGHT],
    pub text_layer: TextLayer,
    pub sprites: Vec<Sprite>,
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
            frame: Box::new([0; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]),
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

    // pub fn get_txt_size_xy(&self) -> (usize, usize) {
    //     self.text_layer.get_dimensions_xy()
    // }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<usize> {
        let index = frame_coord_to_index(x as isize, y as isize);
        if let Some(index_value) = index {
            Some(self.frame[index_value])
        } else {
            None
        }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: usize) {
        let index = frame_coord_to_index(x, y);
        if let Some(index_value) = index {
            self.frame[index_value] = color
        }
    }

    pub fn set_line_scroll_list(&mut self, index: usize, value: isize) {
        if index < self.line_scroll_list.len() {
            self.line_scroll_list[index] = value;
        }
    }

    pub fn set_overscan_color(&mut self, color: usize) {
        self.set_overscan_color_range(color, 0..VIRTUAL_HEIGHT)
    }

    pub fn set_overscan_color_range<R: RangeBounds<usize>>(&mut self, color: usize, range: R) {
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
    pub fn clear(&mut self, color: usize, style: Option<TextCellStyle>) {
        self.frame
            .copy_from_slice(&[color; VIRTUAL_WIDTH * VIRTUAL_HEIGHT]);
        self.overscan.copy_from_slice(&[color; VIRTUAL_HEIGHT]);
        self.text_layer.map = [[TextCell { c: None, style }; TEXT_COLUMNS]; TEXT_ROWS];
    }

    pub fn render(&mut self, output_frame: &mut [u8]) {
        self.clock.update();

        //Sprites
        self.sprite_layer_renderer();

        //Text layer
        self.text_layer_renderer();

        //Line offset
        self.apply_line_scroll_effect();

        //Overscan
        self.overscan_renderer();

        self.render_to_output_frame(output_frame);

        self.clock.count_frame();
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

    /// Gets all the sprites listed in the sprite vector and renders them at the right place in the
    /// the virtual frame buffer
    fn sprite_layer_renderer(&mut self) {
        for sprite in &self.sprites {
            let mut pixel_count = 0;
            let mut sprite_line_count = 0;

            let global_offset = frame_coord_to_index(sprite.pos_x, sprite.pos_y);

            if let Some(global_offset_value) = global_offset {
                for pixel in &sprite.image {
                    let virtual_fb_offset =
                        (global_offset_value + VIRTUAL_WIDTH * sprite_line_count + pixel_count)
                            % (VIRTUAL_WIDTH * VIRTUAL_HEIGHT);

                    if *pixel != 0 {
                        self.frame[virtual_fb_offset] = *pixel;
                    }

                    pixel_count += 1;
                    if pixel_count == sprite.size.size().0 {
                        pixel_count = 0;
                        sprite_line_count += 1;
                    }
                }
            }
        }
    }

    fn text_layer_renderer(&mut self) {
        for line_count in 0..TEXT_ROWS {
            for col_count in 0..TEXT_COLUMNS {
                let text_cell = self.text_layer.map[line_count][col_count];
                let frame_coord = text_coord_to_frame_coord(col_count, line_count);
                self.text_layer_char_renderer(text_cell, frame_coord.0, frame_coord.1);
            }
        }
    }

    fn text_layer_char_renderer(
        &mut self,
        text_layer_cell: text_layer::TextCell,
        frame_x_pos: usize,
        frame_y_pos: usize,
    ) {
        if text_layer_cell.c.is_none() && text_layer_cell.style.is_none() {
            return;
        }

        let c: char = text_layer_cell.c.unwrap_or(' ');
        let style: TextCellStyle = text_layer_cell.style.unwrap_or(DEFAULT_STYLE);
        let char_color = style.color;
        let bck_color = style.bkg_color;
        let blink = style.blink;
        let swap = style.swap_color;
        let shadowed = style.shadowed;

        //set color, swap or not
        let text_color = if swap || (blink && self.clock.half_second_latch) {
            bck_color
        } else {
            char_color
        };
        let text_bkg_color = if swap || (blink && self.clock.half_second_latch) {
            char_color
        } else {
            bck_color
        };

        //Get char picture from  "character rom"
        let pic = rom(c);

        //Draw picture pixel by pixel in frame buffer
        for (row_count, row) in pic.iter().enumerate().take(CHARACTER_HEIGHT) {
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

                mask >>= 1;
            }
        }
    }

    pub fn render_to_output_frame(&self, output_frame: &mut [u8]) {
        let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];

        for (frame_line_count, frame_line) in self.frame.chunks_exact(VIRTUAL_WIDTH).enumerate() {
            for frame_pixel in 0..VIRTUAL_WIDTH {
                let mut rgb = COLOR_PALETTE[frame_line[frame_pixel]];

                if self.is_inside_rounded_corner(frame_pixel, frame_line_count) {
                    rgb = (0, 0, 0)
                };

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

    fn valid_coord(&self, line: usize, column: usize) -> bool {
        line < TEXT_ROWS && column < TEXT_COLUMNS
    }

    pub fn set_char(&mut self, line: usize, column: usize, c: char) {
        if self.valid_coord(line, column) {
            self.text_layer.map[line][column].c = Some(c);
        }
    }

    pub fn set_style(&mut self, line: usize, column: usize, style: TextCellStyle) {
        if self.valid_coord(line, column) {
            self.text_layer.map[line][column].style = Some(style);
        }
    }

    pub fn set_color(&mut self, line: usize, column: usize, color: usize, bkg_color: usize) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.text_layer.map[line][column].style {
                style.color = color;
                style.bkg_color = bkg_color;
            } else {
                let style: TextCellStyle = TextCellStyle {
                    color,
                    bkg_color,
                    swap_color: false,
                    blink: false,
                    shadowed: false,
                    flip_h: false,
                    flip_v: false,
                };
                self.text_layer.map[line][column].style = Some(style);
            }
        }
    }

    pub fn set_swap(&mut self, line: usize, column: usize, swap_color: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.text_layer.map[line][column].style {
                style.swap_color = swap_color;
            }
        }
    }

    pub fn set_blink(&mut self, line: usize, column: usize, blink: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.text_layer.map[line][column].style {
                style.blink = blink;
            }
        }
    }

    pub fn set_shadowed(&mut self, line: usize, column: usize, shadowed: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.text_layer.map[line][column].style {
                style.shadowed = shadowed;
            }
        }
    }

    pub fn set_flip(&mut self, line: usize, column: usize, flip_h: bool, flip_v: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.text_layer.map[line][column].style {
                style.flip_h = flip_h;
                style.flip_v = flip_v;
            }
        }
    }

    /// Inserts a string in the map at the specified x and y position.
    pub fn write(&mut self, line: usize, column: usize, color: usize, bkg_color: usize, text: &str) {
        match text {
            Text::Char(c) => self.set_char(line, column, c),
            Text::String(str) => {
                if !str.is_empty() {
                    for (char_index, c) in str.chars().enumerate() {
                        self.set_char(line, column + char_index, c)
                    }
                }
            }
            Text::StyledChar(c, style) => {
                self.set_char(line, column, c);
                self.set_style(line, column, style);
            }
            Text::StyledString(str, style) => {
                if !str.is_empty() {
                    for (char_index, c) in str.chars().enumerate() {
                        self.set_char(line, column + char_index, c);
                        self.set_style(line, column + char_index, style);
                    }
                }
            }
            Text::ColoredChar(c, color, bkg_color) => {
                let style = TextCellStyle {
                    color,
                    bkg_color,
                    blink: false,
                    swap_color: false,
                    shadowed: false,
                    flip_h: false,
                    flip_v: false,
                };
                self.set_char(line, column, c);
                self.set_style(line, column, style);
            }
            Text::ColoredString(str, color, bkg_color) => {
                let style = TextCellStyle {
                    color,
                    bkg_color,
                    blink: false,
                    swap_color: false,
                    shadowed: false,
                    flip_h: false,
                    flip_v: false,
                };
                if !str.is_empty() {
                    for (char_index, c) in str.chars().enumerate() {
                        self.set_char(line, column + char_index, c);
                        self.set_style(line, column + char_index, style);
                    }
                }
            }
        }
    }

    pub fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: usize) {
        if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                self.set_pixel(x, y1, color);
            }
            return;
        }

        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                self.set_pixel(x1, y, color);
            }
            return;
        }

        let dx: isize = (x2 - x1).abs();
        let dy: isize = -(y2 - y1).abs();
        let sx: isize = if x1 < x2 { 1 } else { -1 };
        let sy: isize = if y1 < y2 { 1 } else { -1 };
        let mut error = dx + dy;

        let mut x0 = x1;
        let mut y0 = y1;
        let x1 = x2;
        let y1 = y2;

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

    pub fn vector(&mut self, x: isize, y: isize, l: isize, color: usize, a: f32) -> (isize, isize) {
        let x1 = x;
        let y1 = y;

        let x_move = a.cos() * l as f32;
        let y_move = a.sin() * l as f32;

        let x2: isize = if x_move < 0.0 {
            x1 - (-x_move).round() as isize
        } else {
            x1 + x_move.round() as isize
        };

        let y2: isize = if y_move < 0.0 {
            y1 - (-y_move).round() as isize
        } else {
            y1 + y_move.round() as isize
        };

        self.line(x1, y1, x2, y2, color);

        (x2, y2)
    }

    pub fn square(&mut self, x: isize, y: isize, width: isize, height: isize, color: usize) {
        self.line(x, y, x + width - 1, y, color);
        self.line(x + width - 1, y, x + width - 1, y + height - 1, color);
        self.line(x + width - 1, y + height - 1, x, y + height - 1, color);
        self.line(x, y + height - 1, x, y, color);
    }

    pub fn fill_square(
        &mut self,
        x: isize,
        y: isize,
        width: isize,
        height: isize,
        color: usize,
        fill_color: usize,
    ) {
        self.square(x, y, width, height, color);
        for y in (y + 1)..(y + height - 1) {
            self.line(x + 1, y, x + width - 2, y, fill_color);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_circle(
        &mut self,
        xc: isize,
        yc: isize,
        x: isize,
        y: isize,
        color: usize,
        fill_color: usize,
        fill: bool,
    ) {
        self.set_pixel(xc + x, yc + y, color);
        self.set_pixel(xc - x, yc + y, color);
        self.set_pixel(xc + x, yc - y, color);
        self.set_pixel(xc - x, yc - y, color);
        self.set_pixel(xc + y, yc + x, color);
        self.set_pixel(xc - y, yc + x, color);
        self.set_pixel(xc + y, yc - x, color);
        self.set_pixel(xc - y, yc - x, color);

        if fill {
            self.line(xc - x, yc + y - 1, xc + x, yc + y - 1, fill_color);
            self.line(xc - x, yc - y + 1, xc + x, yc - y + 1, fill_color);
            self.line(xc - y + 1, yc + x, xc + y - 1, yc + x, fill_color);
            self.line(xc - y + 1, yc - x, xc + y - 1, yc - x, fill_color);
        }
    }

    pub fn circle(
        &mut self,
        xc: isize,
        yc: isize,
        r: usize,
        color: usize,
        fill_color: usize,
        fill: bool,
    ) {
        let mut x: isize = 0;
        let mut y: isize = r as isize;
        let mut d: isize = 3 - 2 * r as isize;

        //Special case for r = 1
        if r == 1 {
            self.set_pixel(xc, yc + 1, color);
            self.set_pixel(xc, yc - 1, color);
            self.set_pixel(xc + 1, yc, color);
            self.set_pixel(xc - 1, yc, color);

            if fill {
                self.set_pixel(xc, yc, fill_color)
            }

            return;
        }

        self.draw_circle(xc, yc, x, y, color, fill_color, fill);

        while y >= x {
            x += 1;

            if d > 0 {
                d = d + 4 * (x - y) + 10;
                y -= 1;
            } else {
                d = d + 4 * x + 6;
            }

            self.draw_circle(xc, yc, x, y, color, fill_color, fill);
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
