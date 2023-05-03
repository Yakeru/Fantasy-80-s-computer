use display_controller::{config::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, OVERSCAN_H, OVERSCAN_V}, color_palettes::*, DisplayController};

use super::{map::{Map, Wall}, player::Player, math::{ray, find_intersection, get_distance_between_points}, texture::{STONE_64X64, Texture}};

pub const FOV: f32 = 1.0;
pub const GAME_SCALE: isize = 256;
pub const MINIMAP_SCALE: isize = 10;
pub const WALL_HEIGHT: isize = 256;

pub struct Renderer {
    pub fov: f32,
    nb_of_rays: isize,
    pub wall_height: isize,
    pub render_distance: isize,
    projection_distance: isize,
    compensated_projection_dist: Vec<isize>,
    wall_depth_buffer: Vec<isize>,
    stone_texture: Texture
}

impl Renderer {

    pub fn new() -> Self {
        Renderer {
            fov: FOV,
            wall_height: WALL_HEIGHT, 
            render_distance: GAME_SCALE * 10, 
            projection_distance: VIRTUAL_WIDTH as isize, 
            compensated_projection_dist: Vec::new(),
            nb_of_rays: VIRTUAL_WIDTH as isize,
            wall_depth_buffer: Vec::new(),
            stone_texture: Texture::new(&STONE_64X64, 64, 64)
        }
    }

    pub fn distortion_compensation(&mut self) {
        let ray_angle_step = self.fov / self.nb_of_rays as f32;
        for ray_count in 0..self.nb_of_rays {
            let ray_angle = self.fov / 2.0 - ray_count as f32 * ray_angle_step;
            let distance_to_projection_surface = self.projection_distance as f32 / ray_angle.cos();
            self.compensated_projection_dist.push(distance_to_projection_surface as isize);
        }
    }

    pub fn draw_sky(&self, dc: &mut DisplayController, color: u8, fog_color: u8) {
        dc.clear(DARK_GREY);
        dc.square(0, 67, VIRTUAL_WIDTH as isize, 18, DARKER_GREY, DARKER_GREY, true);
        dc.square(0, 85, VIRTUAL_WIDTH as isize, 200, DARKER_GREY, DARKER_GREY, true);
        for y in 85..95 {
            if y % 2 == 0 {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if x % 4 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            } else {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if (x+2) % 4 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            }
        }

        for y in 95..101 {
            if y % 2 == 0 {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if x % 2 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            } else {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if (x+1) % 2 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            }
        }

        for y in 101..VIRTUAL_HEIGHT/2 {
            dc.line(0, y as isize, VIRTUAL_WIDTH as isize, y as isize, BLACK)
        }
    }

    pub fn draw_ground(&self, dc: &mut DisplayController, color: u8, fog_color: u8) {
        dc.square(0, (VIRTUAL_HEIGHT/2) as isize , VIRTUAL_WIDTH as isize, (VIRTUAL_HEIGHT/2) as isize, DARK_BROWN, DARK_BROWN, true);

        for y in VIRTUAL_HEIGHT/2..155 {
            dc.line(0, y as isize, VIRTUAL_WIDTH as isize, y as isize, BLACK)
        }

        for y in 155..160 {
            if y % 2 == 0 {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if x % 2 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            } else {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if (x+1) % 2 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            }
        }

        for y in 160..172 {
            if y % 2 == 0 {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if x % 4 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            } else {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if (x+2) % 4 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            }
        }
        dc.square(0, (VIRTUAL_HEIGHT - 67) as isize, VIRTUAL_WIDTH as isize, 200, BROWN, BROWN, true);
    }

    pub fn scale_texture_column(&self, column: &Vec<u8>, new_height: isize) -> Vec<u8> {
        let mut scaled: Vec<u8> = Vec::new();
        
        for pixel in 0..new_height as usize {
            let texture_pixel_index: usize = (pixel * (column.len() - 1)) / new_height as usize;
            let pixel_color = column.get(texture_pixel_index).unwrap();
            scaled.push(*pixel_color);
        }

        scaled
    }

    pub fn shade_texture_column(&self, column: &mut Vec<u8>, dist: isize) {
        if dist > self.render_distance {
            column.fill(0);
        }
    }

    pub fn draw_column(&self, dc: &mut DisplayController, x: isize, data: &Vec<u8>) {
        let line_y_start = (VIRTUAL_HEIGHT/2) as isize - data.len() as isize / 2;

        for pix_in_line in 0..data.len() as isize {
            let pixel_index = (line_y_start + pix_in_line) as usize;
            let mut pixel_color = data[pix_in_line as usize];

            // //Shadow pixel
            // if x % 2 == 0 {
            //     if dist > 7 * GAME_SCALE {if pix_in_line % 2 == 0 {pixel_color = 0} else {pixel_color += 16}}
            //     else if dist > 6 * GAME_SCALE {if pix_in_line % 4 == 0 {pixel_color = 0} else {pixel_color += 16}}
            //     else if dist > 5 * GAME_SCALE { pixel_color += 16 }
            //     else if dist > 4 * GAME_SCALE {if pix_in_line % 2 == 0 {pixel_color += 16}}
            //     else if dist > 3 * GAME_SCALE {if pix_in_line % 4 == 0 {pixel_color += 16}};
            // } else {
            //     if dist > 7 * GAME_SCALE {if (pix_in_line + 1) % 2 == 0 {pixel_color = 0} else {pixel_color += 16}}
            //     else if dist > 6 * GAME_SCALE {if (pix_in_line + 2) % 4 == 0{pixel_color = 0} else {pixel_color += 16}}
            //     else if dist > 5 * GAME_SCALE {pixel_color += 16}
            //     else if dist > 4 * GAME_SCALE {if (pix_in_line + 1) % 2  == 0 {pixel_color += 16}}
            //     else if dist > 3 * GAME_SCALE {if (pix_in_line + 2) % 4 == 0 {pixel_color += 16}};
            // }

            // if dist > 8 * GAME_SCALE {pixel_color = 0};

            dc.set_pixel(x, pixel_index as isize, pixel_color);
        }
    }

    pub fn draw_top_view_map(&self, dc: &mut DisplayController, map: &Map, player: &Player) {
        // Draw player and view cone
        let player_coord =
            convert_map_coord_to_minimap_coord(player.x as isize, player.y as isize);
        dc.circle(
            player_coord.0,
            player_coord.1,
            2 as usize,
            GREEN,
            GREEN,
            true,
        );
        let r0 = dc.vector(
            player_coord.0,
            player_coord.1,
            MINIMAP_SCALE / 2,
            GREEN,
            player.direction - self.fov / 2.0,
        );
        let r319 = dc.vector(
            player_coord.0,
            player_coord.1,
            MINIMAP_SCALE / 2,
            GREEN,
            player.direction + self.fov / 2.0,
        );
        dc.line(r0.0, r0.1, r319.0, r319.1, GREEN);

        //Draw  mini map
        for wall in map.walls.chunks_exact(1) {
            if wall[0].texture == 1 {
                let x1 = wall[0].x1 * MINIMAP_SCALE + OVERSCAN_H as isize;
                let y1 = wall[0].y1 * MINIMAP_SCALE + OVERSCAN_V as isize;
                let x2 = wall[0].x2 * MINIMAP_SCALE + OVERSCAN_H as isize;
                let y2 = wall[0].y2 * MINIMAP_SCALE + OVERSCAN_V as isize;
                dc.line(x1, y1, x2, y2, WHITE);
            }
        }

        //Draw intersection points
        // let view_angle = (self.player.direction + self.settings.fov / 2.0) - (self.player.direction - self.settings.fov / 2.0);
        // let step = view_angle / VIRTUAL_WIDTH as f32;

        // for ray_count in 0..VIRTUAL_WIDTH {
        //     let ray_angle = self.player.direction - self.settings.fov / 2.0 + ray_count as f32 * step;
        //     let ray = vector(self.player.x, self.player.y, ray_angle, 1000);
        //     let mut closest_intersection: Option<(isize, isize, isize)> = None;

        //     for wall in self.map.walls.chunks_exact(1) {
        //         if wall[0].texture == 1 {
        //             let intersection = find_intersection(wall[0].x1, wall[0].y1, wall[0].x2, wall[0].y2, self.player.x, self.player.y, ray.0, ray.1);
        //             if intersection.is_some() {
        //                 if closest_intersection.is_none() || intersection.unwrap().2 < closest_intersection.unwrap().2 {
        //                     closest_intersection = intersection;
        //                 }
        //             }
        //         }
        //     }

        //     if closest_intersection.is_some() {
        //         let intersec_pix = convert_map_coord_to_minimap_coord(closest_intersection.unwrap().0, closest_intersection.unwrap().1);
        //         dc.set_pixel(intersec_pix.0, intersec_pix.1, RED);
        //     }
        // }
    }

    pub fn render(&mut self, dc: &mut DisplayController, map: &Map, player: &Player) {

        self.draw_sky(dc, DARK_GREY, BLACK);
        self.draw_ground(dc, BROWN, BLACK);

        let total_view_angle = (player.direction + self.fov / 2.0) - (player.direction - self.fov / 2.0);
        let ray_angle_step = total_view_angle / self.nb_of_rays as f32;
        let player_x = player.x;
        let player_y = player.y;
        self.wall_depth_buffer.clear();

        for ray_count in 0..self.nb_of_rays {
            let ray_angle = player.direction - self.fov / 2.0 + ray_count as f32 * ray_angle_step;
            let ray = ray(player_x, player_y, ray_angle, self.render_distance);
            let mut closest_intersection: Option<(isize, isize, isize)> = None;
            let mut wall_to_render: &Wall = &Wall::new();

            //Check each wall against ray, keep closest intersection
            for wall in map.walls.chunks_exact(1) {
                let wall_start_x = wall[0].x1 * GAME_SCALE;
                let wall_start_y = wall[0].y1 * GAME_SCALE;
                let wall_end_x = wall[0].x2 * GAME_SCALE;
                let wall_end_y = wall[0].y2 * GAME_SCALE;

                let intersection = find_intersection(
                    wall_start_x,wall_start_y, 
                    wall_end_x, wall_end_y, 
                    player_x, player_y, 
                    ray.0, ray.1);

                if intersection.is_some() {
                    if closest_intersection.is_none() || intersection.unwrap().2 < closest_intersection.unwrap().2 {
                        closest_intersection = intersection;
                        wall_to_render = &wall[0];
                    }
                }
            }

            if closest_intersection.is_some() || wall_to_render.texture != 0 {
                let dist = closest_intersection.unwrap().2;
                self.wall_depth_buffer.push(dist);

                if dist > 0 {
                    // Line height
                    let height = self.wall_height * (self.compensated_projection_dist[ray_count as usize]) / dist;
                                    
                    // Texture mapping
                    
                    // Find position of intersection of wall relative to texture column (64 x 64 texture)
                    let wall_origin_to_ray_intersect_dist = get_distance_between_points(wall_to_render.x1 * GAME_SCALE, wall_to_render.y1 * GAME_SCALE, closest_intersection.unwrap().0, closest_intersection.unwrap().1);
                    let mut texture_column_index = (wall_origin_to_ray_intersect_dist * self.stone_texture.get_width() as isize) / GAME_SCALE;
                    if texture_column_index >= 64 {texture_column_index = 63};
                    let texture_column = self.stone_texture.get_column(texture_column_index as usize);
                    let mut scaled_texture_column = self.scale_texture_column(texture_column, height);
                    self.shade_texture_column(&mut scaled_texture_column, dist);
                    self.draw_column(dc, ray_count, &scaled_texture_column);
                }
            }
        }
    }
}

fn convert_map_coord_to_minimap_coord(x: isize, y: isize) -> (isize, isize) {
    // 0 to GAME_SCALE is equivalent to 0 to MINIMAP_SCALE
    return (
        x / (GAME_SCALE / MINIMAP_SCALE) + OVERSCAN_H as isize,
        y / (GAME_SCALE / MINIMAP_SCALE) + OVERSCAN_V as isize,
    );
}