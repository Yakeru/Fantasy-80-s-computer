use std::f32::consts::PI;

use fantasy_cpc_display_controller::{
    color_palettes::*,
    config::{OVERSCAN_H, OVERSCAN_V, VIRTUAL_HEIGHT, VIRTUAL_WIDTH},
    DisplayController,
};

use crate::apps::raycaster::math::range_conversion;

use super::{
    map::{Map, Wall},
    math::{cast_ray, find_intersection, get_distance_between_points, Segment},
    monster::Monster,
    player::Player,
    texture::Texture,
    textures::stone_wall::STONE_64X64,
};

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
    stone_texture: Texture,
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
            stone_texture: Texture::new(&STONE_64X64, 64, 64),
        }
    }

    pub fn distortion_compensation(&mut self) {
        let ray_angle_step = self.fov / self.nb_of_rays as f32;
        for ray_count in 0..self.nb_of_rays {
            let ray_angle = self.fov / 2.0 - ray_count as f32 * ray_angle_step;
            let distance_to_projection_surface = self.projection_distance as f32 / ray_angle.cos();
            self.compensated_projection_dist
                .push(distance_to_projection_surface as isize);
        }
    }

    pub fn draw_sky(&self, dc: &mut DisplayController, _color: usize, _fog_color: usize) {
        dc.clear(DARK_GREY);
        dc.square(
            0,
            67,
            VIRTUAL_WIDTH as isize,
            18,
            DARKER_GREY,
            DARKER_GREY,
            true,
        );
        dc.square(
            0,
            85,
            VIRTUAL_WIDTH as isize,
            200,
            DARKER_GREY,
            DARKER_GREY,
            true,
        );
        for y in 85..95 {
            if y % 2 == 0 {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if x % 4 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            } else {
                for x in 0..VIRTUAL_WIDTH as isize {
                    if (x + 2) % 4 == 0 {
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
                    if (x + 1) % 2 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            }
        }

        for y in 101..VIRTUAL_HEIGHT / 2 {
            dc.line(0, y as isize, VIRTUAL_WIDTH as isize, y as isize, BLACK)
        }
    }

    pub fn draw_ground(&self, dc: &mut DisplayController, _color: usize, _fog_color: usize) {
        dc.square(
            0,
            (VIRTUAL_HEIGHT / 2) as isize,
            VIRTUAL_WIDTH as isize,
            (VIRTUAL_HEIGHT / 2) as isize,
            DARK_BROWN,
            DARK_BROWN,
            true,
        );

        for y in VIRTUAL_HEIGHT / 2..155 {
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
                    if (x + 1) % 2 == 0 {
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
                    if (x + 2) % 4 == 0 {
                        dc.set_pixel(x, y, BLACK);
                    }
                }
            }
        }
        dc.square(
            0,
            (VIRTUAL_HEIGHT - 67) as isize,
            VIRTUAL_WIDTH as isize,
            200,
            BROWN,
            BROWN,
            true,
        );
    }

    pub fn scale_texture_column(&self, column: &Vec<usize>, new_height: isize) -> Vec<usize> {
        let mut scaled: Vec<usize> = Vec::new();

        for pixel in 0..new_height as usize {
            let texture_pixel_index: usize = (pixel * column.len()) / new_height as usize;
            let pixel_color = column.get(texture_pixel_index).unwrap();
            scaled.push(*pixel_color);
        }

        scaled
    }

    pub fn shade_texture_column(&self, column: &mut [usize], map: &Map, dist: isize, x: isize) {
        if dist > self.render_distance {
            column.fill(map.fog_color);
            return;
        }

        let fog_step = map.fog_range * GAME_SCALE / 6;

        for step in (0..7).rev() {
            let fog_dist = map.fog_distance * GAME_SCALE + step * fog_step;
            if dist > fog_dist {
                match step {
                    6 => self.apply_level_7_shade(column, x),
                    5 => self.apply_level_6_shade(column, x),
                    4 => self.apply_level_5_shade(column, x),
                    3 => self.apply_level_4_shade(column, x),
                    2 => self.apply_level_3_shade(column, x),
                    1 => self.apply_level_2_shade(column, x),
                    0 => self.apply_level_1_shade(column, x),
                    _ => column.fill(map.fog_color),
                }
            }
        }
    }

    fn apply_level_1_shade(&self, column: &mut [usize], x: isize) {
        for pixel in column.iter_mut().enumerate() {
            if (x % 2 == 0 && x % 4 == 0 && pixel.0 % 4 == 0)
                || (x % 2 == 0 && (pixel.0 + 2) % 4 == 0)
            {
                *pixel.1 = 0;
            }
        }
    }

    fn apply_level_2_shade(&self, column: &mut [usize], x: isize) {
        for pixel in column.iter_mut().enumerate() {
            if x % 2 == 0 && x % 4 == 0 && pixel.0 % 2 == 0 {
                *pixel.1 = 0;
            }
        }
    }

    fn apply_level_3_shade(&self, column: &mut [usize], x: isize) {
        for pixel in column.iter_mut().enumerate() {
            if (x % 2 == 0 && x % 4 == 0 && pixel.0 % 4 == 0)
                || ((x + 2) % 2 == 0 && pixel.0 % 2 == 0)
            {
                *pixel.1 = 0;
            }
        }
    }

    fn apply_level_4_shade(&self, column: &mut [usize], x: isize) {
        for pixel in column.iter_mut().enumerate() {
            if (x % 2 == 0 && pixel.0 % 2 == 0) || ((pixel.0 + 1) % 2 == 0) {
                *pixel.1 = 0;
            }
        }
    }

    fn apply_level_5_shade(&self, column: &mut [usize], x: isize) {
        for pixel in column.iter_mut().enumerate() {
            if (x % 2 == 0 && x % 4 == 0 && pixel.0 % 4 == 0)
                || ((x + 1) % 2 == 0 && pixel.0 % 2 == 0)
            {
                *pixel.1 = 0;
            }
        }
    }

    fn apply_level_6_shade(&self, column: &mut [usize], x: isize) {
        for pixel in column.iter_mut().enumerate() {
            if x % 2 == 0 && (x % 4 == 0 || pixel.0 % 2 == 0) {
                *pixel.1 = 0;
            }
        }
    }

    fn apply_level_7_shade(&self, column: &mut [usize], x: isize) {
        for pixel in column.iter_mut().enumerate() {
            if x % 2 == 0 && (x % 4 == 0 || pixel.0 % 4 != 0) {
                *pixel.1 = 0;
            }
        }
    }

    pub fn draw_column(
        &self,
        dc: &mut DisplayController,
        x: isize,
        data: &Vec<usize>,
        transparent_color: usize,
    ) {
        let line_y_start = (VIRTUAL_HEIGHT / 2) as isize - data.len() as isize / 2;
        for pix_in_line in 0..data.len() as isize {
            let pixel_index = (line_y_start + pix_in_line) as usize;
            let pixel_color = data[pix_in_line as usize];
            if pixel_color != transparent_color {
                dc.set_pixel(x, pixel_index as isize, pixel_color);
            }
        }
    }

    pub fn draw_top_view_map(
        &self,
        dc: &mut DisplayController,
        map: &Map,
        player: &Player,
        monster: &Monster,
    ) {
        // Draw player and view cone
        let player_coord = convert_map_coord_to_minimap_coord(player.x, player.y);
        dc.circle(player_coord.0, player_coord.1, 2, GREEN, GREEN, true);
        let r0 = dc.vector(
            player_coord.0,
            player_coord.1,
            MINIMAP_SCALE,
            GREEN,
            player.direction - self.fov / 2.0,
        );
        let r319 = dc.vector(
            player_coord.0,
            player_coord.1,
            MINIMAP_SCALE,
            GREEN,
            player.direction + self.fov / 2.0,
        );
        dc.line(r0.0, r0.1, r319.0, r319.1, GREEN);

        let monster_coord = convert_map_coord_to_minimap_coord(monster.x, monster.y);
        dc.circle(monster_coord.0, monster_coord.1, 2, RED, RED, true);

        //Draw  mini map
        for wall in map.walls.chunks_exact(1) {
            if wall[0].texture == 1 {
                let x1 = wall[0].x1 / GAME_SCALE * MINIMAP_SCALE + OVERSCAN_H as isize;
                let y1 = wall[0].y1 / GAME_SCALE * MINIMAP_SCALE + OVERSCAN_V as isize;
                let x2 = wall[0].x2 / GAME_SCALE * MINIMAP_SCALE + OVERSCAN_H as isize;
                let y2 = wall[0].y2 / GAME_SCALE * MINIMAP_SCALE + OVERSCAN_V as isize;
                dc.line(x1, y1, x2, y2, WHITE);
            }
        }
    }

    pub fn render(
        &mut self,
        dc: &mut DisplayController,
        map: &Map,
        player: &Player,
        monster: &Monster,
    ) {
        self.draw_sky(dc, DARK_GREY, BLACK);
        self.draw_ground(dc, BROWN, BLACK);

        // Project rays
        let total_view_angle =
            (player.direction + self.fov / 2.0) - (player.direction - self.fov / 2.0);
        let ray_angle_step = total_view_angle / self.nb_of_rays as f32;
        self.wall_depth_buffer.clear();

        for ray_count in 0..self.nb_of_rays {
            let ray_angle = player.direction - self.fov / 2.0 + ray_count as f32 * ray_angle_step;
            let ray = cast_ray(player.x, player.y, ray_angle, self.render_distance);

            // Draw walls
            let mut closest_intersection: Option<(isize, isize, isize)> = None;
            let mut wall_to_render: &Wall = &Wall::new();

            //Check each wall against ray, keep closest intersection
            for wall in map.walls.chunks_exact(1) {
                let seg_1: Segment = Segment {
                    x1: wall[0].x1,
                    y1: wall[0].y1,
                    x2: wall[0].x2,
                    y2: wall[0].y2,
                };
                let seg_2: Segment = Segment {
                    x1: player.x,
                    y1: player.y,
                    x2: ray.0,
                    y2: ray.1,
                };
                let intersection = find_intersection(seg_1, seg_2);

                if intersection.is_some()
                    && (closest_intersection.is_none()
                        || intersection.unwrap().2 < closest_intersection.unwrap().2)
                {
                    closest_intersection = intersection;
                    wall_to_render = &wall[0];
                }
            }

            if closest_intersection.is_none() {
                self.wall_depth_buffer.push(self.render_distance);
            }

            if closest_intersection.is_some() || wall_to_render.texture != 0 {
                let dist = closest_intersection.unwrap().2;
                self.wall_depth_buffer.push(dist);

                if dist > 0 {
                    // Line height
                    let height = self.wall_height
                        * (self.compensated_projection_dist[ray_count as usize])
                        / dist;

                    // Texture mapping
                    // Find position of intersection relative to texture
                    let wall_origin_to_ray_intersect_dist = get_distance_between_points(
                        wall_to_render.x1,
                        wall_to_render.y1,
                        closest_intersection.unwrap().0,
                        closest_intersection.unwrap().1,
                    );
                    let mut texture_column_index = (wall_origin_to_ray_intersect_dist
                        * self.stone_texture.get_width() as isize)
                        / GAME_SCALE;
                    if texture_column_index >= 64 {
                        texture_column_index = 63
                    };
                    // Get coresponding texture column, scale it, shade it, draw it
                    let texture_column =
                        self.stone_texture.get_column(texture_column_index as usize);
                    let mut scaled_texture_column =
                        self.scale_texture_column(texture_column, height);
                    self.shade_texture_column(&mut scaled_texture_column, map, dist, ray_count);
                    self.draw_column(dc, ray_count, &scaled_texture_column, 255);
                }
            }
        }

        // If monster is within field of view and within rendering distance
        let monster_test = self.monster_is_in_fov(player, monster);
        if monster_test.0 {
            let dist_monster = monster_test.1;
            let monster_width = monster.size * self.projection_distance / dist_monster;

            for x in (monster_test.2 - monster_width / 2)..=(monster_test.2 + monster_width / 2) {
                if x > 0
                    && x < self.wall_depth_buffer.len() as isize
                    && dist_monster < self.wall_depth_buffer[x as usize]
                {
                    let texture_column_index: isize = range_conversion(
                        (monster_test.2 - monster_width / 2) as f32,
                        (monster_test.2 + monster_width / 2) as f32,
                        x as f32,
                        0.0,
                        63.0,
                    ) as isize;
                    let texture_column = monster.texture.get_column(texture_column_index as usize);
                    let scaled_texture_column =
                        self.scale_texture_column(texture_column, monster_width);
                    self.draw_column(dc, x, &scaled_texture_column, 0);
                }
            }
        }
    }

    fn monster_is_in_fov(&self, player: &Player, monster: &Monster) -> (bool, isize, isize) {
        let dist_monster = get_distance_between_points(player.x, player.y, monster.x, monster.y);
        if dist_monster > self.render_distance {
            return (false, 0, 0);
        }
        if dist_monster == 0 {
            return (false, 0, 0);
        }
        let mut angle_diff = player.direction - monster.angle_to_player;

        //Player direction in Q3 and monster in Q2
        if is_in_q3(player.direction) && is_in_q2(monster.angle_to_player) {
            angle_diff += 2.0 * PI;
        }

        //Player direction in Q2 && monster in Q3
        if is_in_q3(monster.angle_to_player) && is_in_q2(player.direction) {
            angle_diff -= 2.0 * PI;
        }

        let monster_screen_pos =
            range_conversion(0.5, -0.5, angle_diff, 0.0, VIRTUAL_WIDTH as f32) as isize;

        (true, dist_monster, monster_screen_pos)
    }
}

fn convert_map_coord_to_minimap_coord(x: isize, y: isize) -> (isize, isize) {
    (
        (x as f32 / GAME_SCALE as f32 * MINIMAP_SCALE as f32) as isize + OVERSCAN_H as isize,
        (y as f32 / GAME_SCALE as f32 * MINIMAP_SCALE as f32) as isize + OVERSCAN_V as isize,
    )
}

fn is_in_q3(angle: f32) -> bool {
    (-PI..=-PI / 2.0).contains(&angle)
}

fn is_in_q2(angle: f32) -> bool {
    (PI / 2.0..=PI).contains(&angle)
}
