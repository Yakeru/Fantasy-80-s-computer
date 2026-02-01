use crate::{DisplayController, config::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH}, frame_coord_to_index};

impl DisplayController {
    /// Gets all the sprites listed in the sprite vector and renders them at the right place in the
    /// the virtual frame buffer
    pub fn sprite_layer_renderer(&mut self) {
        for sprite in &self.sprites {
            let mut pixel_count = 0;
            let mut sprite_line_count = 0;

            let global_offset = frame_coord_to_index(sprite.pos_x, sprite.pos_y);

            if global_offset.is_some() {
                for pixel in &sprite.image {
                    let virtual_fb_offset =
                        (global_offset.unwrap() + VIRTUAL_WIDTH * sprite_line_count + pixel_count)
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
}