use super::{
    map::Map,
    math::cast_ray,
    monster::Monster,
    player::Player,
    renderer::{Renderer, GAME_SCALE},
};
use fantasy_cpc_app_trait::{FantasyCpcApp, AppStatus};
use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::{color_palettes::{YELLOW, BLACK}, DisplayController};
use fast_math::atan2;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;
use std::f32::consts::PI;

const PLAYER_SPEED: isize = 8;

pub struct Raycaster {
    enable_auto_escape: bool,
    name: String,
    status: AppStatus,
    initialized: bool,
    map: Map,
    renderer: Renderer,
    player: Player,
    monster: Monster,
    show_menu: bool,
    draw_minimap: bool,
    menu_item_selected: usize,
}

impl Raycaster {
    pub fn new() -> Raycaster {
        Raycaster {
            enable_auto_escape: false,
            name: "raycaster".to_string(),
            status: AppStatus::Stopped,
            initialized: false,
            map: Map::new(),
            player: Player {
                x: 0,
                y: 0,
                direction: 0.0,
            },
            monster: Monster::new(),
            show_menu: false,
            draw_minimap: false,
            renderer: Renderer::new(),
            menu_item_selected: 0,
        }
    }

    //***************************************************************************************************************** */
    //                                                    GAME
    //***************************************************************************************************************** */
    pub fn update_game(&mut self, inputs: Option<&WinitInputHelper>, _clock: &Clock) {
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

            if self.player.direction < -PI {
                self.player.direction += 2.0 * PI;
            }
        }

        if inputs.key_held(VirtualKeyCode::Right) {
            self.player.direction += 0.05;

            if self.player.direction > PI {
                self.player.direction -= 2.0 * PI;
            }
        }

        if inputs.key_held(VirtualKeyCode::Up) {
            let move_to = cast_ray(
                self.player.x,
                self.player.y,
                self.player.direction,
                PLAYER_SPEED,
            );

            let col_test = cast_ray(self.player.x, self.player.y, self.player.direction, 50);

            if self.map.get_cell_from_coord(self.player.x, col_test.1) == 0 {
                self.player.y = move_to.1;
            }

            if self.map.get_cell_from_coord(col_test.0, self.player.y) == 0 {
                self.player.x = move_to.0;
            }
        }

        if inputs.key_held(VirtualKeyCode::Down) {
            let move_to = cast_ray(
                self.player.x,
                self.player.y,
                self.player.direction + PI,
                PLAYER_SPEED,
            );

            let col_test = cast_ray(self.player.x, self.player.y, self.player.direction + PI, 50);

            if self.map.get_cell_from_coord(self.player.x, col_test.1) == 0 {
                self.player.y = move_to.1;
            }

            if self.map.get_cell_from_coord(col_test.0, self.player.y) == 0 {
                self.player.x = move_to.0;
            }
        }

        // See monster ?
        let monster_angle: f32 = atan2(
            (self.monster.y - self.player.y) as f32,
            (self.monster.x - self.player.x) as f32,
        );
        self.monster.angle_to_player = monster_angle;
    }


    //***************************************************************************************************************** */
    //                                                    MENU
    //***************************************************************************************************************** */
    pub fn update_menu(&mut self, inputs: Option<&WinitInputHelper>, _clock: &Clock) {
        if inputs.is_none() {
            return;
        }

        let inputs = inputs.unwrap();

        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.show_menu = false;
            return;
        }

        if inputs.key_pressed(VirtualKeyCode::Return) {
            if self.menu_item_selected == 4 {
                self.set_state(AppStatus::Stopped);
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

        if inputs.key_pressed_os(VirtualKeyCode::Up) && self.menu_item_selected > 0 {
            self.menu_item_selected -= 1
        }

        if inputs.key_pressed_os(VirtualKeyCode::Down) && self.menu_item_selected < 4 {
            self.menu_item_selected += 1
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

impl FantasyCpcApp for Raycaster {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_state(&self) -> &AppStatus {
        &self.status
    }

    fn set_state(&mut self, state: AppStatus) {
        self.status = state;
    }

    fn get_initialized(&self) -> bool {
        self.initialized
    }

    fn set_initialized(&mut self, is_initialized: bool) {
        self.initialized = is_initialized
    }

    fn get_enable_autoescape(&self) -> bool {
        self.enable_auto_escape
    }

    fn init_app(&mut self, system_clock: &fantasy_cpc_clock::Clock, display_controller: &mut fantasy_cpc_display_controller::DisplayController) {
        display_controller.get_text_layer_mut().clear();
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

    fn update_app(
        &mut self,
        inputs: Option<&winit_input_helper::WinitInputHelper>,
        clock: &fantasy_cpc_clock::Clock,
    ) -> Option<fantasy_cpc_app_trait::AppResponse> {
        if self.show_menu {
            self.update_menu(inputs, clock);
        } else {
            self.update_game(inputs, clock);
        }

        None
    }

    fn draw_app(&mut self, clock: &fantasy_cpc_clock::Clock, display_controller: &mut fantasy_cpc_display_controller::DisplayController) {
        // Menu
        if self.show_menu {
            self.draw_menu(clock, display_controller);
        }

        // Ray casting renderer
        self.renderer
            .render(display_controller, &self.map, &self.player, &self.monster);

        //Draw minimap
        if self.draw_minimap {
            self.renderer
                .draw_top_view_map(display_controller, &self.map, &self.player, &self.monster);
        }
    }
}