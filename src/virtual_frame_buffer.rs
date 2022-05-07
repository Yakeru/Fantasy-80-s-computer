use crate::color_palettes::default_color_palette;

//Contains a list of u8 values corresponding to values from a color palette.
//So just one u8 per pixel, R G and B values are retrieved from the palette.
//No Alpha for instance.
//This frame buffer is meant to contain a low resolution low color picure that 
//will be upscaled into the final "real" 2D frame buffer.
pub struct VirtualFrameBuffer {
    width: u32,
    height: u32,
    frame: Vec<u8>,
}

impl VirtualFrameBuffer {
    pub fn new(width: u32, height: u32) -> VirtualFrameBuffer {
        let size: u32 = width * height;
        let mut virtual_frame_buffer = Vec::new();

        for _value in 0..size {
            virtual_frame_buffer.push(0);
        }

        VirtualFrameBuffer {
            width: width,
            height: height,
            frame: virtual_frame_buffer
        }
    }

    pub fn get_frame(&mut self) -> &mut [u8] {
        return &mut self.frame;
    }

    //Sets all the pixels to the specified color of the specified palette
    pub fn clear_frame_buffer(&mut self, color: u8) {
        for value in self.frame.chunks_exact_mut(1) {
            value[0] = color;
        }
    }

    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }
}

pub struct CrtEffectRenderer {
    input_frame_px_width: u32, 
    input_frame_px_height: u32,
    output_frame_px_width: u32,
    output_frame_px_height: u32,
    output_nb_of_values_per_pixel: u8,
    render_horiz_upscale: u8,
    render_vert_upscale: u8,
    scan_line_strength: u8,
    sub_pixel_attenuation: u8,
}

//Specifically for this project, the output frame buffer must be at least 3x wider and 4x higher to apply the CRT effect
//Upscalling is fixed to 3x4 with that specific renderer
impl CrtEffectRenderer {

    pub fn new(input_width: u32, input_height: u32, output_width: u32, output_height: u32) -> CrtEffectRenderer {
        CrtEffectRenderer {
            input_frame_px_width: input_width,
            input_frame_px_height: input_height,
            output_frame_px_width: output_width,
            output_frame_px_height: output_height,
            render_horiz_upscale: 3,
            render_vert_upscale: 4,
            output_nb_of_values_per_pixel: 4,
            scan_line_strength: 35,
            sub_pixel_attenuation: 230,
        }
    }

    pub fn render(&self, input_frame: &[u8], output_frame: &mut[u8]) {
        
        let mut virt_line_pixel_counter: usize = 0;
        let mut virt_line_counter: usize = 0;
        let pixels_sub_pixel_count = 4;

        let max_output_index = self.output_frame_px_width * self.output_frame_px_height * self.output_nb_of_values_per_pixel as u32;

        for pixel in input_frame {

            //Temporary color index to RGB mapping
            let rgb: (u8, u8, u8) = default_color_palette(pixel);
            
            //Offset between virtual frame buffer and pixel's frame buffer
            //if scaling is applied, it represents the offset between virtual frame buffer's pixel and
            //pixel's top-left corner of scalled pixel
            let global_offset = pixels_sub_pixel_count * virt_line_pixel_counter * self.render_horiz_upscale as usize 
            + self.output_frame_px_width as usize * pixels_sub_pixel_count * virt_line_counter * self.render_vert_upscale as usize;

            if global_offset < max_output_index as usize {
            
                for horizontal_copy in 0..self.render_horiz_upscale {
                    for vertical_copy in 0..self.render_vert_upscale {
                        let scaling_offset: usize = pixels_sub_pixel_count * horizontal_copy as usize + self.output_frame_px_width as usize * pixels_sub_pixel_count * vertical_copy as usize;
                        let final_offset: usize = global_offset + scaling_offset;
                        let mut final_rgb: (u8, u8, u8) = rgb;

                        match horizontal_copy {
                            0 => {
                                if final_rgb.1 < self.sub_pixel_attenuation {final_rgb.1 = 0} else {final_rgb.1 -= self.sub_pixel_attenuation};
                                if final_rgb.2 < self.sub_pixel_attenuation {final_rgb.2 = 0} else {final_rgb.2 -= self.sub_pixel_attenuation};
                            },
                            1 => {
                                if final_rgb.0 < self.sub_pixel_attenuation {final_rgb.0 = 0} else {final_rgb.0 -= self.sub_pixel_attenuation};
                                if final_rgb.2 < self.sub_pixel_attenuation {final_rgb.2 = 0} else {final_rgb.2 -= self.sub_pixel_attenuation};
                            },
                            2 => {
                                if final_rgb.0 < self.sub_pixel_attenuation {final_rgb.0 = 0} else {final_rgb.0 -= self.sub_pixel_attenuation};
                                if final_rgb.1 < self.sub_pixel_attenuation {final_rgb.1 = 0} else {final_rgb.1 -= self.sub_pixel_attenuation};
                            },
                            _ => {}
                        }

                        if vertical_copy == self.render_vert_upscale - 1 {
                            if final_rgb.0 < self.scan_line_strength {final_rgb.0 = 0} else {final_rgb.0 -= self.scan_line_strength};
                            if final_rgb.1 < self.scan_line_strength {final_rgb.1 = 0} else {final_rgb.1 -= self.scan_line_strength};
                            if final_rgb.2 < self.scan_line_strength {final_rgb.2 = 0} else {final_rgb.2 -= self.scan_line_strength};
                        }

                        output_frame[0 + final_offset] = final_rgb.0;
                        output_frame[1 + final_offset] = final_rgb.1;
                        output_frame[2 + final_offset] = final_rgb.2;
                        output_frame[3 + final_offset] = 254;
                        
                    }
                }

                virt_line_pixel_counter += 1;
                if virt_line_pixel_counter == self.input_frame_px_width as usize {
                    virt_line_pixel_counter = 0;
                    virt_line_counter += 1;
                }
            } else {
                println!("CrtEffectRenderer exceeded the size of the output frame buffer.");
            }
        }
    }
}

// fn simple_upscale_renderer(virtual_fb: &mut[u8], pixels_frame: &mut[u8], integer_width_multiplier: usize, integer_height_multiplier: usize) {
    
//     let mut virt_line_pixel_counter: usize = 0;
//     let mut virt_line_counter: usize = 0;
//     let pixels_sub_pixel_count = 4;

//     for pixel in virtual_fb {

//         //Temporary color index to RGB mapping
//         let mut rgb: (u8, u8, u8) = (0, 0, 0);
//         match pixel {
//             0 => {
//                 rgb.0 = 0;
//                 rgb.1 = 0;
//                 rgb.2 = 0;
//             },
//             1 => {
//                 rgb.0 = 254;
//                 rgb.1 = 254;
//                 rgb.2 = 254;
//             },
//             2 => {
//                 rgb.0 = 254;
//                 rgb.1 = 0;
//                 rgb.2 = 0;
//             },
//             3 => {
//                 rgb.0 = 0;
//                 rgb.1 = 254;
//                 rgb.2 = 0;
//             },
//             4 => {
//                 rgb.0 = 0;
//                 rgb.1 = 0;
//                 rgb.2 = 254;
//             },
//             5 => {
//                 rgb.0 = 254;
//                 rgb.1 = 254;
//                 rgb.2 = 0;
//             },
//             6 => {
//                 rgb.0 = 0;
//                 rgb.1 = 254;
//                 rgb.2 = 254;
//             },
//             7 => {
//                 rgb.0 = 254;
//                 rgb.1 = 0;
//                 rgb.2 = 254;
//             },
//             8.. => {
//                 rgb.0 = 0;
//                 rgb.1 = 0;
//                 rgb.2 = 0;
//             }
//         }

//         //Offset between virtual frame buffer and pixel's frame buffer
//         //if scaling is applied, it represents the offset between virtual frame buffer's pixel and
//         //pixel's top-left corner of scalled pixel
//         let global_offset = pixels_sub_pixel_count * virt_line_pixel_counter * integer_width_multiplier 
//         + WIDTH as usize * pixels_sub_pixel_count * virt_line_counter * integer_height_multiplier;
        
//         for horizontal_copy in 0..integer_width_multiplier {
//             for vertical_copy in 0..integer_height_multiplier {
//                 let scaling_offset:usize = pixels_sub_pixel_count * horizontal_copy + WIDTH as usize * pixels_sub_pixel_count * vertical_copy;
//                 let final_offset: usize = global_offset + scaling_offset;
//                 pixels_frame[0 + final_offset] = rgb.0;
//                 pixels_frame[1 + final_offset] = rgb.1;
//                 pixels_frame[2 + final_offset] = rgb.2;
//                 pixels_frame[3 + final_offset] = 254;
//             }
//         }

//         virt_line_pixel_counter += 1;
//         if virt_line_pixel_counter == VIRTUAL_WIDTH as usize {
//             virt_line_pixel_counter = 0;
//             virt_line_counter += 1;
//         }
//     }
// }