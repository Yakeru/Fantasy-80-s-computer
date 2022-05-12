use crate::color_palettes::default_color_palette;
use crate::text_layer::{TextLayer, TextLayerRenderer};

//Contains a list of u8 values corresponding to values from a color palette.
//So just one u8 per pixel, R G and B values are retrieved from the palette.
//No Alpha.
//This frame buffer is meant to contain a low resolution low color picure that 
//will be upscaled into the final pixel 2D frame buffer.
pub struct VirtualFrameBuffer {
    width: usize,
    height: usize,
    frame: Vec<u8>
}

impl VirtualFrameBuffer {
    pub fn new(fb_width: usize, fb_height: usize, text_columns: usize, text_rows: usize) -> VirtualFrameBuffer {
        let size: usize = fb_width * fb_height;
        let mut virtual_frame_buffer = Vec::new();

        for _value in 0..size {
            virtual_frame_buffer.push(0);
        }

        VirtualFrameBuffer {
            width: fb_width,
            height: fb_height,
            frame: virtual_frame_buffer,
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

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }
}

pub struct CrtEffectRenderer {
    input_frame_px_width: usize, 
    input_frame_px_height: usize,
    output_frame_px_width: usize,
    output_frame_px_height: usize,
    output_nb_of_values_per_pixel: usize,
    render_horiz_upscale: usize,
    render_vert_upscale: usize,
    scan_line_strength: u8,
    sub_pixel_attenuation: u8,
}

//Specifically for this project, the output frame buffer must be at least 3x wider and 4x higher to apply the CRT effect
//Upscalling is fixed to 3x4 with that specific renderer
impl CrtEffectRenderer {

    pub fn new(input_width: usize, input_height: usize, output_width: usize, output_height: usize) -> CrtEffectRenderer {
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

        let max_output_index = self.output_frame_px_width * self.output_frame_px_height * self.output_nb_of_values_per_pixel ;

        for pixel in input_frame {

            //Temporary color index to RGB mapping
            let rgb: (u8, u8, u8) = default_color_palette(pixel);
            
            //Offset between virtual frame buffer and pixel's frame buffer
            //if scaling is applied, it represents the offset between virtual frame buffer's pixel and
            //pixel's top-left corner of scalled pixel
            let global_offset = pixels_sub_pixel_count * virt_line_pixel_counter * self.render_horiz_upscale  
            + self.output_frame_px_width * pixels_sub_pixel_count * virt_line_counter * self.render_vert_upscale ;

            if global_offset < max_output_index  {  
                for horizontal_copy in 0..self.render_horiz_upscale {
                    for vertical_copy in 0..self.render_vert_upscale {
                        let scaling_offset: usize = pixels_sub_pixel_count * horizontal_copy  + self.output_frame_px_width  * pixels_sub_pixel_count * vertical_copy ;
                        let final_offset: usize = global_offset + scaling_offset;
                        let mut final_rgb: (u8, u8, u8) = rgb;

                        // //Use 3 consecutive pixels as the 3 sub components of a single pixel
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

                        //Scanline effect : dim every fourth line
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
                if virt_line_pixel_counter == self.input_frame_px_width  {
                    virt_line_pixel_counter = 0;
                    virt_line_counter += 1;
                }
            } else {
                println!("CrtEffectRenderer exceeded the size of the output frame buffer.");
            }
        }
    }
}