use super::{
    map::Map,
    math::ray,
    player::Player,
    renderer::{Renderer, GAME_SCALE}
};
use app_macro::AppMacro;
use app_macro_derive::AppMacro;
use std::{f32::consts::PI,};

const PLAYER_SPEED: isize = 8;

#[derive(AppMacro)]
pub struct Raycaster {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    map: Map,
    renderer: Renderer,
    player: Player,
    show_menu: bool,
    draw_minimap: bool,
    menu_item_selected: usize,
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
            player: Player {
                x: 0,
                y: 0,
                direction: 0.0,
            },
            show_menu: false,
            draw_minimap: false,
            renderer: Renderer::new(),
            menu_item_selected: 0,
        }
    }

    //***************************************************************************************************************** */
    //                                                    APP
    //***************************************************************************************************************** */
    pub fn init_app(&mut self, _clock: &Clock, dc: &mut DisplayController) {
        dc.get_console_mut().display = false;
        dc.get_text_layer_mut().clear();
        self.map.walls.clear();
        self.show_menu = false;
        self.draw_minimap = false;
        self.map.transform_map_into_list_of_walls();
        self.renderer.distortion_compensation();
        self.player = Player::new(
            self.map.player_start_x * GAME_SCALE + GAME_SCALE / 2,
            self.map.player_start_y * GAME_SCALE + GAME_SCALE / 2,
            self.map.player_start_dir,
        );
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
            return;
        }

        let inputs = inputs.unwrap();

        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.show_menu = true;
            return;
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
            let move_to = ray(
                self.player.x,
                self.player.y,
                self.player.direction,
                PLAYER_SPEED,
            );
            self.player.x = move_to.0;
            self.player.y = move_to.1;
        }

        if inputs.key_held(VirtualKeyCode::Down) {
            let move_to = ray(
                self.player.x,
                self.player.y,
                self.player.direction + PI,
                PLAYER_SPEED,
            );
            self.player.x = move_to.0;
            self.player.y = move_to.1;
        }
    }

    pub fn draw_app(&mut self, clock: &Clock, dc: &mut DisplayController) {
        // Menu
        if self.show_menu {
            self.draw_menu(clock, dc);
        }

        // Ray casting renderer
        self.renderer.render(dc, &self.map, &self.player);

        //Draw minimap
        if self.draw_minimap {
            self.renderer.draw_top_view_map(dc, &self.map, &self.player);
        }
    }

    //***************************************************************************************************************** */
    //                                                    MENU
    //***************************************************************************************************************** */
    pub fn update_menu(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        _clock: &Clock,
        dc: &mut DisplayController,
    ) {
        if inputs.is_none() {
            return;
        }

        let inputs = inputs.unwrap();

        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.show_menu = false;
            dc.get_text_layer_mut().clear();
            return;
        }

        if inputs.key_pressed(VirtualKeyCode::Return) {
            if self.menu_item_selected == 4 {
                self.set_state(false, false);
                self.initialized = false;
            } else {
                self.show_menu = false;
            }
            return;
        }

        if inputs.key_pressed_os(VirtualKeyCode::Left) {
            match self.menu_item_selected {
                0 => {
                    self.renderer.fov -= 0.1;
                    self.renderer.distortion_compensation();
                }
                1 => self.renderer.wall_height -= 10,
                2 => self.renderer.render_distance -= 10,
                _ => (),
            }
        }

        if inputs.key_pressed_os(VirtualKeyCode::Right) {
            match self.menu_item_selected {
                0 => {
                    self.renderer.fov += 0.1;
                    self.renderer.distortion_compensation();
                }
                1 => self.renderer.wall_height += 10,
                2 => self.renderer.render_distance += 10,
                _ => (),
            }
        }

        if inputs.key_pressed_os(VirtualKeyCode::Up) {
            if self.menu_item_selected > 0 {
                self.menu_item_selected -= 1
            }
        }

        if inputs.key_pressed_os(VirtualKeyCode::Down) {
            if self.menu_item_selected < 4 {
                self.menu_item_selected += 1
            }
        }
    }

    pub fn draw_menu(&mut self, _clock: &Clock, dc: &mut DisplayController) {
        dc.get_text_layer_mut().clear();
        dc.get_text_layer_mut().insert_string_xy(
            2,
            10,
            "fov: ",
            Some(YELLOW),
            Some(BLACK),
            false,
            self.menu_item_selected == 0,
            false,
        );
        dc.get_text_layer_mut().insert_string_xy(
            2,
            11,
            "Wall height: ",
            Some(YELLOW),
            Some(BLACK),
            false,
            self.menu_item_selected == 1,
            false,
        );
        dc.get_text_layer_mut().insert_string_xy(
            2,
            12,
            "Render distance: ",
            Some(YELLOW),
            Some(BLACK),
            false,
            self.menu_item_selected == 2,
            false,
        );
        dc.get_text_layer_mut().insert_string_xy(
            2,
            15,
            "Quit game",
            Some(YELLOW),
            Some(BLACK),
            false,
            self.menu_item_selected == 4,
            false,
        );

        dc.get_text_layer_mut().insert_string_xy(
            7,
            10,
            &self.renderer.fov.to_string(),
            Some(YELLOW),
            Some(BLACK),
            false,
            self.menu_item_selected == 0,
            false,
        );
        dc.get_text_layer_mut().insert_string_xy(
            15,
            11,
            &self.renderer.wall_height.to_string(),
            Some(YELLOW),
            Some(BLACK),
            false,
            self.menu_item_selected == 1,
            false,
        );
        dc.get_text_layer_mut().insert_string_xy(
            19,
            12,
            &self.renderer.render_distance.to_string(),
            Some(YELLOW),
            Some(BLACK),
            false,
            self.menu_item_selected == 2,
            false,
        );
    }
}