use std::ops::Range;

use crate::{config::*, VirtualFrameBuffer};

const SUB_PIXEL_COUNT: usize = 4;
const RENDERED_LINE_LENGTH: usize = WIDTH * SUB_PIXEL_COUNT;

pub struct CrtEffectRenderer {
    upscaling: usize,
    filter: bool,
    brightness: u8
}

impl CrtEffectRenderer {
    pub fn new(upscaling: usize, filter: bool, brightness: u8) -> CrtEffectRenderer {
        CrtEffectRenderer {
            upscaling,
            filter,
            brightness
        }
    }

    pub fn set_brightness(&mut self, br: u8) {
        self.brightness = br;
    }

    pub fn toggle_filter(&mut self) {
        self.filter = !self.filter;
    }

    pub fn render(&self, virtual_frame_buffer: &VirtualFrameBuffer, output_frame: &mut [u8]) {
        //list of values to draw rounded corners (nb of pixels to turn of per line in corner)
        let circle_list: [usize; 17] = [17, 14, 12, 10, 9, 8, 7, 6, 5, 4, 3, 3, 2, 2, 1, 1, 1];

        let mut rendered_scanline: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];
        let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = if self.filter { [0; RENDERED_LINE_LENGTH] } else {[255; RENDERED_LINE_LENGTH]};
        let mut rendered_ramp_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];
        let mut line_count: usize = 0;
        
        for virt_line in virtual_frame_buffer
            .get_frame()
            .chunks_exact(VIRTUAL_WIDTH)
        {
            // Check if line is affected by corner
            // if so, adapt range of pixels to render to exclude corner

            let mut range: Range<usize> = 0..VIRTUAL_WIDTH;

            if line_count < circle_list.len() {
                range.start = circle_list[line_count];
                range.end = VIRTUAL_WIDTH - circle_list[line_count];
            }

            if line_count > VIRTUAL_HEIGHT - circle_list.len() {
                range.start = circle_list[VIRTUAL_HEIGHT - line_count];
                range.end = VIRTUAL_WIDTH - circle_list[VIRTUAL_HEIGHT - line_count];
            }

            let mut rgb_before: (u8, u8, u8) = (0, 0, 0);

            for pixel_index in 0..VIRTUAL_WIDTH {

                // Check if we are inside rounded corner, if true set to black else get color
                let rgb = if range.contains(&pixel_index) { 
                    virtual_frame_buffer
                        .color_palette
                        .get_rgb(virt_line[pixel_index]) 
                } else { 
                    (0, 0, 0) 
                };

                let rgb_after: (u8, u8, u8) = if pixel_index < VIRTUAL_WIDTH - 1 {
                    virtual_frame_buffer
                        .color_palette
                        .get_rgb(virt_line[pixel_index + 1])
                } else {
                    (0, 0, 0)
                };
                
                if self.upscaling == 6 {
                    if self.filter {
                        let scanline_alpha =
                            self.brightness.checked_sub(SCAN_LINE_STRENGTH).unwrap_or(0);

                        let r1 = if rgb.0 > rgb_before.0 {
                            rgb.0 - ((rgb.0 - rgb_before.0) / 5)
                        } else if rgb.0 < rgb_before.0 {
                            rgb.0 + ((rgb_before.0 - rgb.0) / 5)
                        } else {
                            rgb.0
                        };

                        let g1 = if rgb.1 > rgb_before.1 {
                            rgb.1 - ((rgb.1 - rgb_before.1) / 5)
                        } else if rgb.1 < rgb_before.1 {
                            rgb.1 + ((rgb_before.1 - rgb.1) / 5)
                        } else {
                            rgb.1
                        };

                        let b1 = if rgb.2 > rgb_before.2 {
                            rgb.2 - ((rgb.2 - rgb_before.2) / 5)
                        } else if rgb.2 < rgb_before.2 {
                            rgb.2 + ((rgb_before.2 - rgb.2) / 5)
                        } else {
                            rgb.2
                        };

                        let r2 = if rgb.0 > rgb_after.0 {
                            rgb.0 - ((rgb.0 - rgb_after.0) / 5)
                        } else if rgb.0 < rgb_after.0 {
                            rgb.0 + ((rgb_after.0 - rgb.0) / 5)
                        } else {
                            rgb.0
                        };

                        let g2 = if rgb.1 > rgb_after.1 {
                            rgb.1 - ((rgb.1 - rgb_after.1) / 5)
                        } else if rgb.1 < rgb_after.1 {
                            rgb.1 + ((rgb_after.1 - rgb.1) / 5)
                        } else {
                            rgb.1
                        };

                        let b2 = if rgb.2 > rgb_after.2 {
                            rgb.2 - ((rgb.2 - rgb_after.2) / 5)
                        } else if rgb.2 < rgb_after.2 {
                            rgb.2 + ((rgb_after.2 - rgb.2) / 5)
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

                        rendered_scanline[r1_index] = r1 >> 1;
                        rendered_scanline[ar1_index] = scanline_alpha;
                        rendered_scanline[g1_index] = g1 >> 1;
                        rendered_scanline[ag1_index] = scanline_alpha;
                        rendered_scanline[b1_index] = b1 >> 1;
                        rendered_scanline[ab1_index] = scanline_alpha;
                        rendered_scanline[r2_index] = r2 >> 1;
                        rendered_scanline[ar2_index] = scanline_alpha;
                        rendered_scanline[g2_index] = g2 >> 1;
                        rendered_scanline[ag2_index] = scanline_alpha;
                        rendered_scanline[b2_index] = b2 >> 1;
                        rendered_scanline[ab2_index] = scanline_alpha;

                        //--------------------------------------------------------------------------------------

                        rendered_ramp_line[r1_index] = r1 >> 1;
                        rendered_ramp_line[ar1_index] = self.brightness;
                        rendered_ramp_line[g1_index] = g1 >> 1;
                        rendered_ramp_line[ag1_index] = self.brightness;
                        rendered_ramp_line[b1_index] = b1 >> 1;
                        rendered_ramp_line[ab1_index] = self.brightness;
                        rendered_ramp_line[r2_index] = r2 >> 1;
                        rendered_ramp_line[ar2_index] = self.brightness;
                        rendered_ramp_line[g2_index] = g2 >> 1;
                        rendered_ramp_line[ag2_index] = self.brightness;
                        rendered_ramp_line[b2_index] = b2 >> 1;
                        rendered_ramp_line[ab2_index] = self.brightness;

                        //--------------------------------------------------------------------------------------

                        rendered_line[r1_index] = r1;
                        rendered_line[ar1_index] = self.brightness;
                        rendered_line[g1_index] = g1;
                        rendered_line[ag1_index] = self.brightness;
                        rendered_line[b1_index] = b1;
                        rendered_line[ab1_index] = self.brightness;
                        rendered_line[r2_index] = r2;
                        rendered_line[ar2_index] = self.brightness;
                        rendered_line[g2_index] = g2;
                        rendered_line[ag2_index] = self.brightness;
                        rendered_line[b2_index] = b2;
                        rendered_line[ab2_index] = self.brightness;

                        rgb_before = rgb;
                    } else {

                        let r = rgb.0;
                        let g = rgb.1;
                        let b = rgb.2;

                        let mut i: usize = 0;
                        while i < SUB_PIXEL_COUNT * self.upscaling {

                            rendered_line[i + SUB_PIXEL_COUNT * UPSCALE * pixel_index] = r;
                            rendered_line[i + SUB_PIXEL_COUNT * UPSCALE * pixel_index + 1] = g;
                            rendered_line[i + SUB_PIXEL_COUNT * UPSCALE * pixel_index + 2] = b;

                            i += SUB_PIXEL_COUNT;
                        }
                    }
                } else  if self.upscaling == 3 {
                    if self.filter {
                        let scanline_alpha =
                            self.brightness.checked_sub(SCAN_LINE_STRENGTH).unwrap_or(0);

                        let r = rgb.0;
                        let r_index = 0 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                        let ar_index = 3 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                        let g = rgb.1;
                        let g_index = 5 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                        let ag_index = 7 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                        let b = rgb.2;
                        let b_index = 10 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;
                        let ab_index = 11 + SUB_PIXEL_COUNT * UPSCALE * pixel_index;

                        rendered_scanline[r_index] = r;
                        rendered_scanline[ar_index] = scanline_alpha;
                        rendered_scanline[g_index] = g;
                        rendered_scanline[ag_index] = scanline_alpha;
                        rendered_scanline[b_index] = b;
                        rendered_scanline[ab_index] = scanline_alpha;

                        //--------------------------------------------------------------------------------------

                        rendered_line[r_index] = r;
                        rendered_line[ar_index] = self.brightness;
                        rendered_line[g_index] = g;
                        rendered_line[ag_index] = self.brightness;
                        rendered_line[b_index] = b;
                        rendered_line[ab_index] = self.brightness;
                    } else {
                        let r = rgb.0;
                        let g = rgb.1;
                        let b = rgb.2;

                        let mut i: usize = 0;
                        while i < SUB_PIXEL_COUNT * self.upscaling {

                            rendered_line[i + SUB_PIXEL_COUNT * UPSCALE * pixel_index] = r;
                            rendered_line[i + SUB_PIXEL_COUNT * UPSCALE * pixel_index + 1] = g;
                            rendered_line[i + SUB_PIXEL_COUNT * UPSCALE * pixel_index + 2] = b;

                            i += SUB_PIXEL_COUNT;
                        }
                    }
                } else {
                    let r = rgb.0;
                    let r_index = 0 + SUB_PIXEL_COUNT * pixel_index;

                    let g = rgb.1;
                    let g_index = 1 + SUB_PIXEL_COUNT * pixel_index;

                    let b = rgb.2;
                    let b_index = 2 + SUB_PIXEL_COUNT * pixel_index;

                    let a = self.brightness;
                    let a_index = 3 + SUB_PIXEL_COUNT * pixel_index;

                    rendered_line[r_index] = r;
                    rendered_line[g_index] = g;
                    rendered_line[b_index] = b;
                    rendered_line[a_index] = a;
                }
            }

            let start = line_count * UPSCALE * RENDERED_LINE_LENGTH;
            if self.upscaling == 6 {
                if self.filter {
                    output_frame[start..start + RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_scanline);
                    output_frame[start + RENDERED_LINE_LENGTH..start + 2 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_ramp_line);
                    output_frame[start + 2 * RENDERED_LINE_LENGTH..start + 3 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_line);
                    output_frame[start + 3 * RENDERED_LINE_LENGTH..start + 4 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_line);
                    output_frame[start + 4 * RENDERED_LINE_LENGTH..start + 5 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_ramp_line);
                    output_frame[start + 5 * RENDERED_LINE_LENGTH..start + 6 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_scanline);
                } else {
                    output_frame[start..start + RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                    output_frame[start + RENDERED_LINE_LENGTH..start + 2 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_line);
                    output_frame[start + 2 * RENDERED_LINE_LENGTH..start + 3 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_line);
                    output_frame[start + 3 * RENDERED_LINE_LENGTH..start + 4 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_line);
                    output_frame[start + 4 * RENDERED_LINE_LENGTH..start + 5 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_line);
                    output_frame[start + 5 * RENDERED_LINE_LENGTH..start + 6 * RENDERED_LINE_LENGTH]
                        .copy_from_slice(&rendered_line);
                }
            } else if self.upscaling == 3 {
                if self.filter {
                    output_frame[start..start + RENDERED_LINE_LENGTH].copy_from_slice(&rendered_line);
                    output_frame[start + RENDERED_LINE_LENGTH..start + 2 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                    output_frame[start + 2 * RENDERED_LINE_LENGTH..start + 3 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_scanline);
                } else {
                    output_frame[start..start + RENDERED_LINE_LENGTH].copy_from_slice(&rendered_line);
                    output_frame[start + RENDERED_LINE_LENGTH..start + 2 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                    output_frame[start + 2 * RENDERED_LINE_LENGTH..start + 3 * RENDERED_LINE_LENGTH]
                    .copy_from_slice(&rendered_line);
                }
            } else {
                output_frame[start..start + RENDERED_LINE_LENGTH].copy_from_slice(&rendered_line); 
            }
            
            line_count += 1;
        }
    }
}