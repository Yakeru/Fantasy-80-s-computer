use crate::{config::*, color_palettes::COLOR_PALETTE};

const SUB_PIXEL_COUNT: usize = 4;
const RENDERED_LINE_LENGTH: usize = VIRTUAL_WIDTH * SUB_PIXEL_COUNT;
const ROUNDED_CORNER: [usize;10] = [10, 8, 6, 5, 4, 3, 2, 2, 1, 1];

pub struct Renderer {
    brightness: u8
}

impl Renderer {
    pub fn new(brightness: u8) -> Renderer {
        Renderer {
            brightness
        }
    }

    pub fn set_brightness(&mut self, br: u8) {
        self.brightness = br;
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

    pub fn render(&self, virtual_frame: &[u8], output_frame: &mut [u8]) {

        let mut rendered_line: [u8; RENDERED_LINE_LENGTH] = [0; RENDERED_LINE_LENGTH];

        let mut frame_line_count: usize = 0;

        for frame_line in virtual_frame.chunks_exact(VIRTUAL_WIDTH) {

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
}
