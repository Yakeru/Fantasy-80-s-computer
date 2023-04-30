use std::{f32::consts::{PI}, cmp::{max, min}};

use app_macro::AppMacro;
use app_macro_derive::AppMacro;

use super::map::Map;

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
    wall_height: isize,
    render_distance: isize,
    eye_to_screen_dist: isize,
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
            player: Player { x: 150, y: 450, direction: 0.0 },
            show_menu: false,
            draw_minimap: false,
            settings: Settings { fov: PI/2.0, wall_height: 100, render_distance: 1000, eye_to_screen_dist: 100 },
            menu_item_selected: 0
        }
    }

    pub fn init_app(&mut self, _clock: &Clock, dc: &mut DisplayController) {
        dc.get_console_mut().display = false;
        dc.get_text_layer_mut().clear();
        self.map.walls.clear();
        self.player = Player { x: 150, y: 450, direction: 0.0 };
        self.show_menu = false;
        self.draw_minimap = false; 
        self.map.transform_map_into_list_of_walls();
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
            self.player.direction -= 0.04;
        }

        if inputs.key_held(VirtualKeyCode::Right) {
            self.player.direction += 0.04;
        }

        if inputs.key_held(VirtualKeyCode::Up) {
            let move_to = vector(self.player.x, self.player.y, self.player.direction, 2);
            self.player.x = move_to.0;
            self.player.y = move_to.1;
        }

        if inputs.key_held(VirtualKeyCode::Down) {
            let move_to = vector(self.player.x, self.player.y, self.player.direction + PI, 2);
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
        let view_angle = (self.player.direction + self.settings.fov / 2.0) - (self.player.direction - self.settings.fov / 2.0);
        let step = view_angle / VIRTUAL_WIDTH as f32;

        for ray_count in 0..VIRTUAL_WIDTH {
            let ray_angle = self.player.direction - self.settings.fov / 2.0 + ray_count as f32 * step;
            let ray = vector(self.player.x, self.player.y, ray_angle, 1000);
            let mut closest_intersection: Option<(isize, isize, isize)> = None;

            for wall in self.map.walls.chunks_exact(1) {
                if wall[0].texture == 1 {
                    let intersection = find_intersection(wall[0].x1, wall[0].y1, wall[0].x2, wall[0].y2, self.player.x, self.player.y, ray.0, ray.1);
                    if intersection.is_some() {
                        if closest_intersection.is_none() || intersection.unwrap().2 < closest_intersection.unwrap().2 {
                            closest_intersection = intersection;
                        }
                    }
                }
            }

            if closest_intersection.is_some() {
                let dist = closest_intersection.unwrap().2;
                
                if dist > 0 {
                    // Line height (Thales theorem)
                    let height = self.settings.wall_height * 100 / dist;
                                    
                    // Line color
                    let color = LIGHT_GREY;

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
        clock: &Clock,
        dc: &mut DisplayController,
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
                0 => self.settings.fov -= 0.1,
                1 => self.settings.wall_height -= 10,
                2 => self.settings.render_distance -= 10,
                _ => ()
            }
        }

        if inputs.key_pressed_os(VirtualKeyCode::Right) {
            match self.menu_item_selected {
                0 => self.settings.fov += 0.1,
                1 => self.settings.wall_height += 10,
                2 => self.settings.render_distance += 10,
                3 => self.settings.eye_to_screen_dist += 10,
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
        dc.get_text_layer_mut().insert_string_xy(23, 13, &self.settings.eye_to_screen_dist.to_string(), Some(YELLOW), Some(BLACK), false, self.menu_item_selected == 3, false);
    }

    fn draw_top_view_map(&mut self, dc: &mut DisplayController) {

        // Draw player and view cone
        let player_coord = convert_map_coord_to_minimap_coord(self.player.x as isize, self.player.y as isize);
        dc.circle(player_coord.0, player_coord.1, 2, GREEN, BLUE, true);
        let r0 = dc.vector(player_coord.0, player_coord.1, 20, BLUE, self.player.direction - self.settings.fov / 2.0);
        let r319 = dc.vector(player_coord.0, player_coord.1, 20, BLUE, self.player.direction + self.settings.fov / 2.0);
        dc.line(r0.0, r0.1, r319.0, r319.1, BLUE);

        //Draw  mini map
        for wall in self.map.walls.chunks_exact(1) {
            if wall[0].texture == 1 {
                let x1 = wall[0].x1 / 5 + OVERSCAN_H as isize;
                let y1 = wall[0].y1 / 5 + OVERSCAN_V as isize;
                let x2 = wall[0].x2 / 5 + OVERSCAN_H as isize;
                let y2 = wall[0].y2 / 5 + OVERSCAN_V as isize;
                dc.line(x1, y1, x2, y2, GREEN);
            }
        }

        //Draw intersection points
        let view_angle = (self.player.direction + self.settings.fov / 2.0) - (self.player.direction - self.settings.fov / 2.0);
        let step = view_angle / VIRTUAL_WIDTH as f32;

        for ray_count in 0..VIRTUAL_WIDTH {
            let ray_angle = self.player.direction - self.settings.fov / 2.0 + ray_count as f32 * step;
            let ray = vector(self.player.x, self.player.y, ray_angle, 1000);
            let mut closest_intersection: Option<(isize, isize, isize)> = None;

            for wall in self.map.walls.chunks_exact(1) {
                if wall[0].texture == 1 {
                    let intersection = find_intersection(wall[0].x1, wall[0].y1, wall[0].x2, wall[0].y2, self.player.x, self.player.y, ray.0, ray.1);
                    if intersection.is_some() {
                        if closest_intersection.is_none() || intersection.unwrap().2 < closest_intersection.unwrap().2 {
                            closest_intersection = intersection;
                        }
                    }
                }
            }

            if closest_intersection.is_some() {
                let intersec_pix = convert_map_coord_to_minimap_coord(closest_intersection.unwrap().0, closest_intersection.unwrap().1);
                dc.set_pixel(intersec_pix.0, intersec_pix.1, RED);
            }
        }
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
    return (x / 5 + OVERSCAN_H as isize, y / 5 + OVERSCAN_V as isize) 
}