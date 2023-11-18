use std::time::Instant;

use fantasy_cpc_app_trait::{AppResponse, AppStatus, FantasyCpcApp, FantasyCppAppDefaultParams};
use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::{
    color_palettes::*,
    config::{TEXT_COLUMNS, TEXT_ROWS},
    DisplayController,
};
use rand::Rng;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Life {
    app_params: FantasyCppAppDefaultParams,
    gen_past: [[bool; TEXT_COLUMNS]; TEXT_ROWS],
    gen_a: Box<[[Cell; TEXT_COLUMNS]; TEXT_ROWS]>,
    gen_b: Box<[[Cell; TEXT_COLUMNS]; TEXT_ROWS]>,
    toggle_gen: bool,
    last_update: Instant,
    welcome_screen: bool,
    game: bool,
    menu: bool,
    alive: bool,
    team_a_color: usize,
    team_b_color: usize,
    random_game_mode: bool,
    color_themes: Vec<Vec<usize>>,
    current_theme: usize,
}

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
    age: u8,
    team: Team,
}

#[derive(Clone, Copy)]
enum Team {
    NA,
    A,
    B,
}

impl FantasyCpcApp for Life {
    fn get_app_params(&mut self) -> &mut FantasyCppAppDefaultParams {
        &mut self.app_params
    }

    fn init_app(&mut self, _system_clock: &Clock, display_controller: &mut DisplayController) {
        display_controller.set_brightness(255);
        self.welcome_screen = true;
        self.game = false;
        self.menu = false;
    }

    fn update_app(
        &mut self,
        inputs: Option<&winit_input_helper::WinitInputHelper>,
        _clock: &Clock,
    ) -> Option<AppResponse> {
        if self.welcome_screen {
            self.update_welcome_screen(inputs);
        } else if self.game {
            self.update_game(inputs);
        } else {
            self.update_menu(inputs);
        }

        None
    }

    fn draw_app(&mut self, clock: &Clock, display_controller: &mut DisplayController) {
        if self.welcome_screen {
            self.draw_welcome_screen(clock, display_controller);
        } else if self.game {
            self.draw_game(display_controller);
        } else if self.menu {
            self.draw_menu(display_controller);
        }
    }
}

impl Life {
    pub fn new() -> Life {
        let fire = vec![RED, DARK_ORANGE, ORANGE, YELLOW, LIGHT_YELLOW, WHITE];
        let ice = vec![LAVENDER, BLUE, TRUE_BLUE, LIGHT_GREY, WHITE];
        let nature = vec![BROWN, BLUE, GREEN, LIGHT_GREY];
        let brazil = vec![YELLOW, GREEN, BLUE, WHITE];
        let france = vec![BLUE, WHITE, RED];
        let crazy = vec![LIME_GREEN, RED, GREEN, BLUE, YELLOW, MAUVE, BLUE_GREEN];

        Life {
            app_params: FantasyCppAppDefaultParams::new(String::from("life"), false),
            gen_past: [[false; TEXT_COLUMNS]; TEXT_ROWS],
            gen_a: Box::new(
                [[Cell {
                    alive: false,
                    age: 0,
                    team: Team::NA,
                }; TEXT_COLUMNS]; TEXT_ROWS],
            ),
            gen_b: Box::new(
                [[Cell {
                    alive: false,
                    age: 0,
                    team: Team::NA,
                }; TEXT_COLUMNS]; TEXT_ROWS],
            ),
            toggle_gen: true,
            last_update: Instant::now(),
            alive: true,
            welcome_screen: true,
            game: false,
            menu: false,
            team_a_color: 8,
            team_b_color: 28,
            random_game_mode: true,
            color_themes: vec![fire, ice, nature, brazil, france, crazy],
            current_theme: 0,
        }
    }

    // Randomizes gen_a. gen_b is emptied,
    // Sets everything back to show gen_a and calculate gen_b
    // chooses a random color theme
    fn restart_sim(&mut self) {
        //Init gen_b with dead cells
        self.gen_b = Box::new(
            [[Cell {
                alive: false,
                age: 0,
                team: Team::NA,
            }; TEXT_COLUMNS]; TEXT_ROWS],
        );

        let mut random = rand::thread_rng();

        //For each cell in gen_a, randomize life.
        //If game mode, cells on the left will be team A, on the right: team B
        for row in 0..TEXT_ROWS {
            for col in 0..TEXT_COLUMNS {
                if self.random_game_mode {
                    self.gen_a[row][col] = Cell {
                        alive: random.gen_range(0..2) != 0,
                        age: 0,
                        team: Team::NA,
                    };
                } else {
                    let cell: Cell = if col < TEXT_COLUMNS / 2 {
                        Cell {
                            alive: random.gen_range(0..2) != 0,
                            age: 0,
                            team: Team::A,
                        }
                    } else {
                        Cell {
                            alive: random.gen_range(0..2) != 0,
                            age: 0,
                            team: Team::B,
                        }
                    };
                    self.gen_a[row][col] = cell;
                }
            }
        }
        self.alive = true;
        self.toggle_gen = true;
        self.current_theme = random.gen_range(0..self.color_themes.len());
    }

    /*************************************************************************************************************
    **************************************************************************************************************
                                                    WELCOME SCREEN
    *************************************************************************************************************
    **************************************************************************************************************/

    fn update_welcome_screen(&mut self, inputs: Option<&WinitInputHelper>) {
        if inputs.is_none() {
            return;
        }
        let user_inputs = inputs.unwrap();

        if user_inputs.key_pressed(VirtualKeyCode::Escape) {
            self.get_app_params().change_status(AppStatus::Stopped)
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key1) {
            self.welcome_screen = false;
            self.menu = false;
            self.game = true;
            self.random_game_mode = true;
            self.restart_sim();
        }

        if user_inputs.key_pressed(VirtualKeyCode::Key2) {
            self.welcome_screen = false;
            self.menu = true;
            self.game = false;
            self.random_game_mode = false;
            self.restart_sim();
        }
    }

    fn draw_welcome_screen(&mut self, clock: &Clock, dc: &mut DisplayController) {
        dc.get_txt_mut().clear();
        dc.clear(BLACK);

        //Animated title
        let title_extraa: &str = " 🯆                         🯆 ";
        let tilte_main_1: &str = " 🯆  Conway's Game Of Life  🯆 ";
        let tilte_main_2: &str = "🯆🯆🯆 Conway's Game Of Life 🯆🯆🯆";
        let title_pos_x: usize = (TEXT_COLUMNS - tilte_main_2.chars().count()) / 2;

        dc.get_txt_mut().set_pen_colors(BLUE, BLACK);

        if clock.second_latch && clock.half_second_latch {
            dc.get_txt_mut().write_str(title_pos_x, 10, title_extraa);
            dc.get_txt_mut().write_str(title_pos_x, 11, tilte_main_1);
            dc.get_txt_mut().write_str(title_pos_x, 12, title_extraa);
        } else if clock.second_latch && !clock.half_second_latch {
            dc.get_txt_mut().write_str(title_pos_x, 11, tilte_main_2);
        } else if !clock.second_latch && clock.half_second_latch {
            dc.get_txt_mut().write_str(title_pos_x, 10, title_extraa);
            dc.get_txt_mut().write_str(title_pos_x, 11, tilte_main_1);
            dc.get_txt_mut().write_str(title_pos_x, 12, title_extraa);
        } else {
            dc.get_txt_mut().write_str(title_pos_x, 11, tilte_main_2);
        }

        // Menu
        let menu_1: &str = "1 - Random mode";
        let menu_1_pos_x: usize = (TEXT_COLUMNS - menu_1.len()) / 2;
        let menu_2: &str = "2 - Combat mode";
        let menu_2_pos_x: usize = (TEXT_COLUMNS - menu_2.len()) / 2;
        dc.get_txt_mut().set_pen_color(ORANGE);
        dc.get_txt_mut().write_str(menu_1_pos_x, 20, menu_1);
        dc.get_txt_mut().write_str(menu_2_pos_x, 21, menu_2);

        // Credit
        let credit: &str = "2022 - Damien Torreilles";
        let credit_pos_x: usize = (TEXT_COLUMNS - credit.len()) / 2;
        dc.get_txt_mut().set_pen_color(TRUE_BLUE);
        dc.get_txt_mut().write_str(credit_pos_x, 29, credit);
    }

    /*************************************************************************************************************
    **************************************************************************************************************
                                                    GAME
    *************************************************************************************************************
    **************************************************************************************************************/

    fn update_game(&mut self, inputs: Option<&WinitInputHelper>) {
        if inputs.is_some() && inputs.unwrap().key_pressed(VirtualKeyCode::C) {
            self.restart_sim();
        }

        if inputs.is_some() && inputs.unwrap().key_pressed(VirtualKeyCode::Escape) {
            self.get_app_params().set_initialized(false);
        }

        let now = Instant::now();

        if now.duration_since(self.last_update).as_millis() >= 50 {
            // Calculate gen_b from gen_a, else calculate gen_b from gen_a
            if self.toggle_gen {
                self.alive = calculate_life(
                    &mut self.gen_past,
                    &mut self.gen_a,
                    &mut self.gen_b,
                    self.random_game_mode,
                );
                self.toggle_gen = !self.toggle_gen;
            } else {
                self.alive = calculate_life(
                    &mut self.gen_past,
                    &mut self.gen_b,
                    &mut self.gen_a,
                    self.random_game_mode,
                );
                self.toggle_gen = !self.toggle_gen;
            }

            self.last_update = Instant::now();

            if !self.alive {
                self.restart_sim();
            }
        }
    }

    fn draw_game(&mut self, dc: &mut DisplayController) {
        dc.get_txt_mut().clear();
        dc.clear(WHITE);
        dc.get_txt_mut().set_pen_bkg_color(BLACK);

        let chars = ['🯆', '🯅', '🯇', '🯈'];

        for col in 0..TEXT_COLUMNS {
            for row in 0..TEXT_ROWS {
                //render gen_a else render gen_b
                let cell: Cell = if self.toggle_gen {
                    self.gen_a[row][col]
                } else {
                    self.gen_b[row][col]
                };

                if cell.alive {
                    let color: usize;
                    if self.random_game_mode {
                        let theme = self.color_themes.get(self.current_theme).unwrap();
                        let color_index = self.gen_a[row][col].age % theme.len() as u8;
                        color = *theme.get(color_index as usize).unwrap();
                    } else {
                        match cell.team {
                            Team::NA => color = 0,
                            Team::A => color = self.team_a_color,
                            Team::B => color = self.team_b_color,
                        }
                    }

                    let char = chars[(self.gen_a[row][col].age % (chars.len() - 1) as u8) as usize];
                    dc.get_txt_mut().set_pen_color(color);
                    dc.get_txt_mut().write(col, row, char);
                } else {
                    dc.get_txt_mut().set_pen_color(BLACK);
                    dc.get_txt_mut().write(col, row, ' ');
                }
            }
        }
    }

    /*************************************************************************************************************
    **************************************************************************************************************
                                                    MENU
    *************************************************************************************************************
    **************************************************************************************************************/

    fn update_menu(&mut self, inputs: Option<&WinitInputHelper>) {
        if inputs.is_none() {
            return;
        }
        let user_inputs = inputs.unwrap();

        if user_inputs.key_pressed(VirtualKeyCode::Escape) {
            self.welcome_screen = true;
            self.menu = false;
            self.game = false;
        }

        if user_inputs.key_pressed(VirtualKeyCode::Left) {
            self.team_a_color -= 1;
        }

        if user_inputs.key_pressed(VirtualKeyCode::Right) {
            self.team_a_color += 1;
        }

        if user_inputs.key_pressed(VirtualKeyCode::Up) {
            self.team_b_color += 1;
        }

        if user_inputs.key_pressed(VirtualKeyCode::Down) {
            self.team_b_color -= 1;
        }

        if user_inputs.key_pressed(VirtualKeyCode::Return) {
            self.welcome_screen = false;
            self.menu = false;
            self.game = true;
        }

        if self.team_a_color == 0 || self.team_a_color >= 31 {
            self.team_a_color = 1
        }
        if self.team_b_color == 0 || self.team_b_color >= 31 {
            self.team_b_color = 1
        }
    }

    fn draw_menu(&mut self, dc: &mut DisplayController) {
        dc.get_txt_mut().clear();
        dc.clear(BLACK);
        dc.get_txt_mut().set_pen_colors(BLUE, BLACK);
        dc.get_txt_mut().write_str(5, 5, "Team A : ");
        dc.get_txt_mut().write_str(5, 7, "Team B : ");
        dc.get_txt_mut().set_pen_colors(self.team_a_color, BLACK);
        dc.get_txt_mut().write(14, 5, '🯆');
        dc.get_txt_mut().set_pen_colors(self.team_b_color, BLACK);
        dc.get_txt_mut().write(14, 7, '🯆');
    }
}

/*************************************************************************************************************
**************************************************************************************************************
                                                VARIOUS FUNCTIONS
**************************************************************************************************************
**************************************************************************************************************/

/// Conway's Game of Life
/// Returns false if stuck in infinite loop, true if things are still dying and birthing
fn calculate_life(
    previous_gen: &mut [[bool; TEXT_COLUMNS]; TEXT_ROWS],
    current_gen: &mut [[Cell; TEXT_COLUMNS]; TEXT_ROWS],
    next_gen: &mut [[Cell; TEXT_COLUMNS]; TEXT_ROWS],
    random_game_mode: bool,
) -> bool {
    let mut _death_count = 0;
    let mut _birth_count = 0;
    let mut _stillborn_count = 0;

    for row in 0..TEXT_ROWS {
        for col in 0..TEXT_COLUMNS {
            let mut a_team_count = 0;
            let mut b_team_count = 0;
            let mut total_count = 0;
            let current_cell = current_gen[row][col];
            let dead_cell = Cell {
                alive: false,
                team: Team::NA,
                age: 0,
            };
            let mut next_gen_cell = current_cell;

            //For each of the 8 cells arround current_gen[row][col]
            #[allow(clippy::needless_range_loop)]
            for row_test in (if row == 0 { 0 } else { row - 1 })..(if row == TEXT_ROWS - 1 {
                TEXT_ROWS - 1
            } else {
                row + 2
            }) {
                for col_test in (if col == 0 { 0 } else { col - 1 })..(if col == TEXT_COLUMNS - 1 {
                    TEXT_COLUMNS - 1
                } else {
                    col + 2
                }) {
                    if !(col_test == col && row_test == row) {
                        let neighbour_cell = current_gen[row_test][col_test];
                        if neighbour_cell.alive {
                            total_count += 1;

                            match neighbour_cell.team {
                                Team::NA => (),
                                Team::A => a_team_count += 1,
                                Team::B => b_team_count += 1,
                            }
                        }
                    }
                }
            }

            if current_cell.alive && !(2..=3).contains(&total_count) {
                next_gen_cell = dead_cell;
                if current_cell.age == 0 {
                    _stillborn_count += 1;
                }
                _death_count += 1;
            } else if !current_cell.alive && total_count == 3 {
                next_gen_cell.alive = true;
                next_gen_cell.age = 0;
                if random_game_mode {
                    next_gen_cell.team = Team::NA;
                } else if a_team_count > b_team_count {
                    next_gen_cell.team = Team::A;
                } else {
                    next_gen_cell.team = Team::B;
                }

                _birth_count += 1;
            } else if current_cell.alive {
                if current_cell.age == 255 {
                    next_gen_cell = dead_cell;
                } else {
                    next_gen_cell.age += 1;
                }
            }

            next_gen[row][col] = next_gen_cell;
        }
    }

    //Compare previous gen with nextgen. If identical, simulation has arrived to a final state
    //Return false if simulation arrived to a final state
    let mut continue_game: bool = false;

    for row in 0..TEXT_ROWS {
        for col in 0..TEXT_COLUMNS {
            if previous_gen[row][col] != next_gen[row][col].alive {
                continue_game = true;
            }
        }
    }

    //Set previous generation from current gen for next update
    for row in 0..TEXT_ROWS {
        for col in 0..TEXT_COLUMNS {
            previous_gen[row][col] = current_gen[row][col].alive;
        }
    }

    continue_game
}