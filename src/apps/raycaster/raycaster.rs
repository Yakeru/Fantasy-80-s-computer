use std::{f32::consts::{PI}, cmp::{max, min}};
use app_macro::AppMacro;
use app_macro_derive::AppMacro;
use super::map::Map;

const FOV: f32 = 1.0;
const GAME_SCALE: isize = 300;
const WALL_HEIGHT: isize = 340;
const MINIMAP_SCALE: isize = 10;
const PLAYER_SPEED: isize = 9;

#[derive(AppMacro)]
pub struct Raycaster {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    map: Map,
    settings: Settings,
    player: Player,
    show_menu: bool,
    draw_minimap: bool,
    menu_item_selected: usize
}

struct Settings {
    fov: f32,
    nb_of_rays: isize,
    wall_height: isize,
    render_distance: isize,
    projection_distance: isize,
    compensated_projection_dist: Vec<isize>
}

impl Settings {

    fn new() -> Self {
        Settings {
            fov: FOV,
            wall_height: WALL_HEIGHT, 
            render_distance: GAME_SCALE * 10, 
            projection_distance: VIRTUAL_HEIGHT as isize, 
            compensated_projection_dist: Vec::new(), 
            nb_of_rays: VIRTUAL_WIDTH as isize 
        }
    }

    fn compute_distortion_compensated_projection_distance(&mut self) {
        self.compensated_projection_dist.clear();
        let ray_angle_step = self.fov / self.nb_of_rays as f32;
        for ray_count in 0..self.nb_of_rays {
            let ray_angle = self.fov / 2.0 - ray_count as f32 * ray_angle_step;
            let distance_to_projection_surface = self.projection_distance as f32 / ray_angle.cos();
            self.compensated_projection_dist.push(distance_to_projection_surface as isize);
        }
    }
}

struct Player {
    x: isize,
    y: isize,
    direction: f32
}

impl Raycaster {
    pub fn new() -> Raycaster {
        Raycaster {
            enable_auto_escape: false,
            name: "raycaster".to_string(),
            updating: false,
            drawing: false,
            initialized: false,
            map: Map::new(),
            player: Player { x: 0, y: 0, direction: 0.0 },
            show_menu: false,
            draw_minimap: false,
            settings: Settings::new(),
            menu_item_selected: 0
        }
    }

    pub fn init_app(&mut self, _clock: &Clock, dc: &mut DisplayController) {
        dc.get_console_mut().display = false;
        dc.get_text_layer_mut().clear();
        self.map.walls.clear();
        self.show_menu = false;
        self.draw_minimap = false; 
        self.map.transform_map_into_list_of_walls();
        self.settings.compute_distortion_compensated_projection_distance();
        self.player = Player { x: self.map.player_start_x * GAME_SCALE + GAME_SCALE/2, 
            y: self.map.player_start_y * GAME_SCALE + GAME_SCALE/2, 
            direction: self.map.player_start_dir };
    }

    pub fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        clock: &Clock,
        dc: &mut DisplayController,
    ) -> Option<AppResponse> {

        if self.show_menu {
            self.update_menu(inputs, clock, dc);
        } else {
            self.update_game(inputs, clock, dc);
        }

        return None;
    }

    //***************************************************************************************************************** */
    //                                                    GAME
    //***************************************************************************************************************** */

    pub fn update_game(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        _clock: &Clock,
        _dc: &mut DisplayController,
    ) {
        if inputs.is_none() {
            return
        }

        let inputs = inputs.unwrap();

        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.show_menu = true;
            return
        }

        if inputs.key_pressed(VirtualKeyCode::M) {
            self.draw_minimap = !self.draw_minimap;
        }

        if inputs.key_held(VirtualKeyCode::Left) {
            self.player.direction -= 0.05;
        }

        if inputs.key_held(VirtualKeyCode::Right) {
            self.player.direction += 0.05;
        }

        if inputs.key_held(VirtualKeyCode::Up) {
            let move_to = vector(self.player.x, self.player.y, self.player.direction, PLAYER_SPEED);
            self.player.x = move_to.0;
            self.player.y = move_to.1;
        }

        if inputs.key_held(VirtualKeyCode::Down) {
            let move_to = vector(self.player.x, self.player.y, self.player.direction + PI, PLAYER_SPEED);
            self.player.x = move_to.0;
            self.player.y = move_to.1;
        }
    }

    pub fn draw_app(
        &mut self,
        clock: &Clock,
        dc: &mut DisplayController,
    ) {
         //Clear screen
         dc.clear(BLACK);
         dc.get_text_layer_mut().clear();

        // Sky and ground
        dc.square(0, 0, VIRTUAL_WIDTH as isize, (VIRTUAL_HEIGHT/2) as isize, BLUE, BLUE, true);
        dc.square(0, (VIRTUAL_HEIGHT/2) as isize, VIRTUAL_WIDTH as isize, VIRTUAL_HEIGHT as isize, DARK_GREEN, DARK_GREEN, true);

        //The menu is drawn over the game, to see changes in real time
        if self.show_menu {
            self.draw_menu(clock, dc);
        }

        //Cast rays and find intersections
        let total_view_angle = (self.player.direction + self.settings.fov / 2.0) - (self.player.direction - self.settings.fov / 2.0);
        let ray_angle_step = total_view_angle / self.settings.nb_of_rays as f32;
        let player_x = self.player.x;
        let player_y = self.player.y;

        for ray_count in 0..self.settings.nb_of_rays {
            let ray_angle = self.player.direction - self.settings.fov / 2.0 + ray_count as f32 * ray_angle_step;
            let ray = vector(player_x, player_y, ray_angle, self.settings.render_distance);
            let mut closest_intersection: Option<(isize, isize, isize)> = None;
            let mut is_edge: bool = false;

            for wall in self.map.walls.chunks_exact(1) {
                if wall[0].texture == 1 {
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

                            let x1_diff = max(wall_start_x, intersection.unwrap().0) - min(wall_start_x, intersection.unwrap().0);
                            let y1_diff = max(wall_start_y, intersection.unwrap().1) - min(wall_start_y, intersection.unwrap().1);
                            let x2_diff = max(wall_end_x, intersection.unwrap().0) - min(wall_end_x, intersection.unwrap().0);
                            let y2_diff = max(wall_end_y, intersection.unwrap().1) - min(wall_end_y, intersection.unwrap().1);

                            if (x1_diff <= 2 && y1_diff <= 2) || (x2_diff <= 2 && y2_diff <= 2) {
                                is_edge = true;
                            } else {
                                is_edge = false;
                            }
                        }
                    }
                }
            }

            if closest_intersection.is_some() {
                let dist = closest_intersection.unwrap().2;
                
                if dist > 0 {
                    // Line height (Thales theorem)
                    let height = self.settings.wall_height * (self.settings.compensated_projection_dist[ray_count as usize]) / dist;
                                    
                    // Line color
                    let color = if is_edge {BLACK} else {LIGHT_GREY};

                    // Draw line
                    dc.line(ray_count as isize, (VIRTUAL_HEIGHT/2) as isize - height/2 as isize, ray_count as isize, (VIRTUAL_HEIGHT/2) as isize + height/2, color);
                }
            }
        }

        //Draw top view
        if self.draw_minimap {
            self.draw_top_view_map(dc);
        }
    }

    //***************************************************************************************************************** */
    //                                                    MENU
    //***************************************************************************************************************** */
    pub fn update_menu(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        _clock: &Clock,
        _dc: &mut DisplayController,
    ) {
        if inputs.is_none() {
            return
        }

        let inputs = inputs.unwrap();

        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.show_menu = false;
            return
        }

        if inputs.key_pressed(VirtualKeyCode::Return) {
            if self.menu_item_selected == 4 {
                self.set_state(false, false);
                self.initialized = false;
            } else {
                self.show_menu = false;
            }
            return
        }

        if inputs.key_pressed_os(VirtualKeyCode::Left) {
            match self.menu_item_selected {
                0 => {
                    self.settings.fov -= 0.1;
                    self.settings.compute_distortion_compensated_projection_distance();
                }
                1 => self.settings.wall_height -= 10,
                2 => self.settings.render_distance -= 10,
                3 => {
                    self.settings.projection_distance -= 10;
                    self.settings.compute_distortion_compensated_projection_distance();
                }
                _ => ()
            }
        }

        if inputs.key_pressed_os(VirtualKeyCode::Right) {
            match self.menu_item_selected {
                0 => {
                    self.settings.fov += 0.1;
                    self.settings.compute_distortion_compensated_projection_distance();
                }
                1 => self.settings.wall_height += 10,
                2 => self.settings.render_distance += 10,
                3 => {
                    self.settings.projection_distance += 10;
                    self.settings.compute_distortion_compensated_projection_distance();
                }
                _ => ()
            }
        }

        if inputs.key_pressed_os(VirtualKeyCode::Up) {
            if self.menu_item_selected > 0 {self.menu_item_selected -= 1}
        }

        if inputs.key_pressed_os(VirtualKeyCode::Down) {
            if self.menu_item_selected < 4 {self.menu_item_selected += 1}
        }
    }

    pub fn draw_menu(
        &mut self,
        _clock: &Clock,
        dc: &mut DisplayController,
    ) {
        dc.get_text_layer_mut().insert_string_xy(2, 10, "fov: ", Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 0, false);
        dc.get_text_layer_mut().insert_string_xy(2, 11, "Wall height: ", Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 1, false);
        dc.get_text_layer_mut().insert_string_xy(2, 12, "Render distance: ", Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 2, false);
        dc.get_text_layer_mut().insert_string_xy(2, 13, "Projection distance: ", Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 3, false);
        dc.get_text_layer_mut().insert_string_xy(2, 15, "Quit game", Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 4, false);

        dc.get_text_layer_mut().insert_string_xy(7, 10, &self.settings.fov.to_string(), Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 0, false);
        dc.get_text_layer_mut().insert_string_xy(15, 11, &self.settings.wall_height.to_string(), Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 1, false);
        dc.get_text_layer_mut().insert_string_xy(19, 12, &self.settings.render_distance.to_string(), Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 2, false);
        dc.get_text_layer_mut().insert_string_xy(23, 13, &self.settings.projection_distance.to_string(), Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 3, false);
    }

    fn draw_top_view_map(&mut self, dc: &mut DisplayController) {

        // Draw player and view cone
        let player_coord = convert_map_coord_to_minimap_coord(self.player.x as isize, self.player.y as isize);
        dc.circle(player_coord.0, player_coord.1, 2 as usize, GREEN, GREEN, true);
        let r0 = dc.vector(player_coord.0, player_coord.1, MINIMAP_SCALE/2, GREEN, self.player.direction - self.settings.fov / 2.0);
        let r319 = dc.vector(player_coord.0, player_coord.1, MINIMAP_SCALE/2, GREEN, self.player.direction + self.settings.fov / 2.0);
        dc.line(r0.0, r0.1, r319.0, r319.1, GREEN);

        //Draw  mini map
        for wall in self.map.walls.chunks_exact(1) {
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
}

/// Returns the intersection of two segments, and the distance between (x1, y1)
/// and the interecting point. None if no interesction.
/// See https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
fn find_intersection(x1: isize, y1: isize, x2: isize, y2: isize, x3: isize, y3: isize, x4: isize, y4: isize) -> Option<(isize, isize, isize)> {
    
    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denom == 0 { return None }
    
    let t_num = (x1 - x3) * (y3- y4) - (y1 - y3) * (x3 - x4);
    let t: f32 = t_num as f32 / denom as f32;
    if t < 0.0 || t > 1.0 { return None }

    let u_num = (x1 - x3) * (y1- y2) - (y1 - y3) * (x1 - x2);
    let u: f32 = u_num as f32 / denom as f32;
    if u < 0.0 || u > 1.0 { return None }

    let result_x = (x3 as f32 + u * (x4 - x3) as f32) as isize;
    let result_y = (y3 as f32 + u * (y4 - y3) as f32) as isize;

    let x_dist = max(x3, result_x) - min(x3, result_x);
    let y_dist = max(y3, result_y) - min(y3, result_y);

    //Distance
    let mut distance = (x_dist * x_dist + y_dist * y_dist) as f64;
    distance = distance.sqrt();
    
    return Some((result_x as isize, result_y as isize, distance as isize));
}

fn vector(x: isize, y: isize, angle_rad:f32, lenght: isize) -> (isize, isize) {
    
    let x1 = x;
    let y1 = y;

    let x_move = angle_rad.cos() * lenght as f32;
    let y_move = angle_rad.sin() * lenght as f32;

    let x2: isize;
    
    if x_move < 0.0 {
        x2 = x1 - (-x_move).round() as isize;
    } else {
        x2 = x1 + x_move.round() as isize;
    }

    let y2: isize;
   
    if y_move < 0.0 {
        y2 = y1 - (-y_move).round() as isize;
    } else {
        y2 = y1 + y_move.round() as isize;
    }

    return (x2, y2)
}

fn convert_map_coord_to_minimap_coord(x: isize, y: isize) -> (isize, isize) {
    // 0 to GAME_SCALE is equivalent to 0 to MINIMAP_SCALE
    return (x / (GAME_SCALE / MINIMAP_SCALE) + OVERSCAN_H as isize, y / (GAME_SCALE / MINIMAP_SCALE) + OVERSCAN_V as isize)
}