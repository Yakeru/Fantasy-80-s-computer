use crate::color_palettes::*;
use crate::text_layer::TextLayer;
use crate::characters_rom::rom;
use crate::sprite::Sprite;
use std::time::{
    Instant, Duration
};

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
    default_text_color: u8,
    default_text_bkg_color: u8,
    frame: Vec<u8>,
    text_layer: TextLayer,
    sprites: Vec<Sprite>,
    mouse_sprite: Sprite,
    //background_layer
    //tiles_layer
    frame_counter: usize,
    second_tick: bool,
    half_second_tick: bool,
    half_second_latch: bool
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

        let mut mouse_sprite: Sprite = Sprite::new_from_file(String::from("mouse"), &String::from("./resources/sprites/sprite1.txt"));
        mouse_sprite.pos_x = fb_width / 2;
        mouse_sprite.pos_y = fb_height / 2;
        
        //TODO init background_layers, tiles_layers, sprites_layers... and correesponding renderes

        VirtualFrameBuffer {
            frame_time_ms,
            width: fb_width,
            height: fb_height,
            columns_count,
            rows_count,
            default_text_color,
            default_text_bkg_color,
            frame: virtual_frame_buffer,
            text_layer,
            sprites,
            mouse_sprite,
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

    pub fn get_mouse_sprite(&mut self) -> &mut Sprite {
        &mut self.mouse_sprite
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

    /// Gets all the sprites listed in the sprite vector and renders them at the rignt place in the
    /// pixel vector of the virtual frame buffer
    fn sprite_layer_renderer(&mut self) {

        for sprite in self.sprites.chunks_exact_mut(1) {

            let mut pixel_count = 0;
            let mut sprite_line_count = 0;

            let global_offset = self.width * sprite[0].pos_y + sprite[0].pos_x;

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

        let mut pixel_count = 0;
            let mut sprite_line_count = 0;

            let global_offset = self.width * self.mouse_sprite.pos_y + self.mouse_sprite.pos_x;

            for pixel in &self.mouse_sprite.image {
        
                let virtual_fb_offset = (global_offset + self.width * sprite_line_count + pixel_count) % (self.width * self.height);

                if *pixel != 0 {
                    self.frame[virtual_fb_offset] = *pixel;
                }
    
                pixel_count += 1;
                if pixel_count == self.mouse_sprite.value_in_physical_size().width {
                    pixel_count = 0;
                    sprite_line_count += 1;
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

                    let mut text_color: u8 = self.default_text_color;
                    let mut text_bkg_color: u8 = self.default_text_bkg_color;

                    match text_mode_char.color {
                        Some(color) => {
                            if text_mode_char.flipp { text_bkg_color = color } else { text_color = color }
                        }
                        None => ()
                    }

                    match text_mode_char.background_color {
                        Some(bkg_color) => {
                            if text_mode_char.flipp { text_color = bkg_color } else { text_bkg_color = bkg_color }
                        }
                        None => ()
                    }

                    //Blink
                    if text_mode_char.blink && self.half_second_latch {
                        text_color = text_bkg_color;
                    }

                    let pic = rom(&text_mode_char.unicode);

                    for row_count in 0..8 {
            
                        let row = pic[row_count];
                        let row_in_binary = &format!("{:0>8b}", row);
                        let mut character_sprite_col_count = 0;
            
                        for c in row_in_binary.chars() {
                            let virtual_frame_buffer_pos = x_pos + character_sprite_col_count + (y_pos + row_count ) * self.width;

                            match c {
                                '0' => self.frame[virtual_frame_buffer_pos] = if text_mode_char.flipp {text_color} else {text_bkg_color},
                                '1' => self.frame[virtual_frame_buffer_pos] = if self.half_second_latch && text_mode_char.blink {text_bkg_color} else { if text_mode_char.flipp {text_bkg_color} else {text_color}},
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
            scan_line_strength: 35,
            sub_pixel_attenuation: 230,
        }
    }

    pub fn render(&self, virtual_frame_buffer: &VirtualFrameBuffer, output_frame: &mut[u8]) {
        
        let mut virt_line_pixel_counter: usize = 0;
        let mut virt_line_counter: usize = 0;
        let pixels_sub_pixel_count = 4;

        let max_output_index = self.output_frame_px_width * self.output_frame_px_height * self.output_nb_of_values_per_pixel ;

        for pixel in virtual_frame_buffer.get_frame_static() {

            //Temporary color index to RGB mapping
            let rgb: (u8, u8, u8) = get_rgb(pixel);
            
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
                if virt_line_pixel_counter == virtual_frame_buffer.get_width() {
                    virt_line_pixel_counter = 0;
                    virt_line_counter += 1;
                }
            } else {
                println!("CrtEffectRenderer exceeded the size of the output frame buffer.");
            }
        }
    }
}