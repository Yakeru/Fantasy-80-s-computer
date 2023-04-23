use std::time::Instant;

use app_macro_derive::AppMacro;
use rand::Rng;
use display_controller::{
    color_palettes::*,
    config::{TEXT_COLUMNS, TEXT_ROWS},
    DisplayController,
};

#[derive(AppMacro)]
pub struct Life {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
    gen_past: [[bool; TEXT_COLUMNS]; TEXT_ROWS],
    gen_a: Box<[[Cell; TEXT_COLUMNS]; TEXT_ROWS]>,
    gen_b: Box<[[Cell; TEXT_COLUMNS]; TEXT_ROWS]>,
    toggle_gen: bool,
    last_update: Instant,
    welcome_screen: bool,
    game: bool,
    menu: bool,
    alive: bool,
    team_a_color: u8,
    team_b_color: u8,
    random_game_mode: bool,
    color_themes: Vec<Vec<u8>>,
    current_theme: usize
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

impl Life {
    pub fn new() -> Life {
        let fire = vec![RED, DARK_ORANGE, ORANGE, YELLOW, LIGHT_YELLOW, WHITE];
        let ice = vec![LAVENDER, BLUE, TRUE_BLUE, LIGHT_GREY, WHITE];
        let nature = vec![BROWN, BLUE, GREEN, LIGHT_GREY];
        let brazil = vec![YELLOW, GREEN, BLUE, WHITE];
        let france = vec![BLUE, WHITE, RED];
        let crazy = vec![LIME_GREEN, RED, GREEN, BLUE, YELLOW, MAUVE, BLUE_GREEN];

        Life {
            enable_auto_escape: false,
            name: String::from("life"),
            updating: false,
            drawing: false,
            initialized: false,
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
            current_theme: 0
        }
    }

    pub fn init_app(&mut self, _clock: &Clock, _display_controller: &mut DisplayController) {
        self.welcome_screen = true;
        self.game = false;
        self.menu = false;
    }

    pub fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        clock: &Clock,
        display_controller: &mut DisplayController,
    ) -> Option<AppResponse> {
        if self.welcome_screen {
            self.update_welcome_screen(inputs, display_controller);
        } else if self.game {
            self.update_game(inputs, clock, display_controller);
        } else {
            self.update_menu(inputs, display_controller);
        }

        return None;
    }

    pub fn draw_app(
        &mut self,
        clock: &Clock,
        display_controller: &mut DisplayController,
    ) {
        if self.welcome_screen {
            self.draw_welcome_screen(clock, display_controller);
        } else if self.game {
            self.draw_game(display_controller);
        } else if self.menu {
            self.draw_menu(display_controller);
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
                    let cell: Cell;
                    if col < TEXT_COLUMNS / 2 {
                        cell = Cell {
                            alive: random.gen_range(0..2) != 0,
                            age: 0,
                            team: Team::A,
                        };
                    } else {
                        cell = Cell {
                            alive: random.gen_range(0..2) != 0,
                            age: 0,
                            team: Team::B,
                        };
                    }
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

    fn update_welcome_screen(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        _display_controller: &mut DisplayController,
    ) {
        if inputs.is_none() {return}
        let user_inputs = inputs.unwrap();

        if user_inputs.key_pressed(VirtualKeyCode::Escape) {
            self.set_state(false, false);
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

    fn draw_welcome_screen(
        &mut self,
        clock: &Clock,
        display_controller: &mut DisplayController,
    ) {
        display_controller.get_text_layer_mut().clear();
        display_controller.get_console_mut().display = false;
        display_controller.clear(BLACK);
        if clock.second_latch && clock.half_second_latch {
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                10,
                " 🯆                         🯆 ",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                11,
                " 🯆  Conway's Game Of Life  🯆 ",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                12,
                " 🯆                         🯆 ",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
        } else if clock.second_latch && !clock.half_second_latch {
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                11,
                "🯆🯆🯆 Conway's Game Of Life 🯆🯆🯆",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
        } else if !clock.second_latch && clock.half_second_latch {
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                10,
                " 🯆                         🯆 ",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                11,
                " 🯆  Conway's Game Of Life  🯆 ",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                12,
                " 🯆                         🯆 ",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
        } else {
            display_controller.get_text_layer_mut().insert_string_xy(
                (TEXT_COLUMNS - 29) / 2,
                11,
                "🯆🯆🯆 Conway's Game Of Life 🯆🯆🯆",
                Some(BLUE),
                Some(BLACK),
                false,
                false,
                false,
            );
        }
        display_controller.get_text_layer_mut().insert_string_xy(
            (TEXT_COLUMNS - 20) / 2,
            20,
            "1 - Random mode",
            Some(ORANGE),
            Some(BLACK),
            false,
            false,
            false,
        );
        display_controller.get_text_layer_mut().insert_string_xy(
            (TEXT_COLUMNS - 20) / 2,
            21,
            "2 - Combat mode",
            Some(ORANGE),
            Some(BLACK),
            false,
            false,
            false,
        );
        display_controller.get_text_layer_mut().insert_string_xy(
            (TEXT_COLUMNS - 24) / 2,
            TEXT_ROWS - 1,
            "2022 - Damien Torreilles",
            Some(TRUE_BLUE),
            Some(BLACK),
            false,
            false,
            false,
        );
    }

    /*************************************************************************************************************
    **************************************************************************************************************
                                                    GAME
    *************************************************************************************************************
    **************************************************************************************************************/

    fn update_game(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        clock: &Clock,
        display_controller: &mut DisplayController,
    ) {
        if inputs.is_some() && inputs.unwrap().key_pressed(VirtualKeyCode::C) {
            self.restart_sim();
        }

        if inputs.is_some() && inputs.unwrap().key_pressed(VirtualKeyCode::Escape) {
            self.init_app(clock, display_controller);
        }

        let now = Instant::now();

        if now.duration_since(self.last_update).as_millis() >= 50 {
            // Calculate gen_b from gen_a, else calculate gen_b from gen_a
            if self.toggle_gen {
                self.alive =
                    calculate_life(&mut self.gen_past, &mut self.gen_a, &mut self.gen_b, self.random_game_mode);
                self.toggle_gen = !self.toggle_gen;
            } else {
                self.alive =
                    calculate_life(&mut self.gen_past, &mut self.gen_b, &mut self.gen_a, self.random_game_mode);
                self.toggle_gen = !self.toggle_gen;
            }

            self.last_update = Instant::now();

            if !self.alive {
                self.restart_sim();
            }
        }
    }

    fn draw_game(&mut self, display_controller: &mut DisplayController) {
        display_controller.get_text_layer_mut().clear();
        display_controller.get_console_mut().display = false;
        display_controller.clear(WHITE);

        let bkg_color = Some(BLACK);

        let chars = ['🯆', '🯅', '🯇', '🯈'];

        for col in 0..TEXT_COLUMNS {
            for row in 0..TEXT_ROWS {
                let cell: Cell;

                //render gen_a else render gen_b
                if self.toggle_gen {
                    cell = self.gen_a[row][col];
                } else {
                    cell = self.gen_b[row][col];
                }

                if cell.alive {
                    let color: Option<u8>;
                    if self.random_game_mode {
                        let theme = self.color_themes.get(self.current_theme as usize).unwrap();
                        let color_index = self.gen_a[row][col].age % theme.len() as u8;
                        color = Some(*theme.get(color_index as usize).unwrap());
                    } else {
                        match cell.team {
                            Team::NA => color = Some(0),
                            Team::A => color = Some(self.team_a_color),
                            Team::B => color = Some(self.team_b_color),
                        }
                    }

                    let char = chars[(self.gen_a[row][col].age % (chars.len() - 1) as u8) as usize];
                    display_controller
                        .get_text_layer_mut()
                        .insert_char_xy(col, row, char, color, bkg_color, false, false, false);
                } else {
                    display_controller
                        .get_text_layer_mut()
                        .insert_char_xy(col, row, ' ', bkg_color, bkg_color, false, false, false);
                }
            }
        }
    }

    /*************************************************************************************************************
    **************************************************************************************************************
                                                    MENU
    *************************************************************************************************************
    **************************************************************************************************************/

    fn update_menu(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        _display_controller: &mut DisplayController,
    ) {

        if inputs.is_none() {return}
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

    fn draw_menu(&mut self, display_controller: &mut DisplayController) {
        display_controller.get_text_layer_mut().clear();
        display_controller.clear(BLACK);
        display_controller.get_text_layer_mut().insert_string_xy(
            5,
            5,
            "Team A : ",
            Some(BLUE),
            Some(BLACK),
            false,
            false,
            false,
        );
        display_controller.get_text_layer_mut().insert_string_xy(
            5,
            7,
            "Team B : ",
            Some(BLUE),
            Some(BLACK),
            false,
            false,
            false,
        );
        display_controller.get_text_layer_mut().insert_string_xy(
            14,
            5,
            "🯆",
            Some(self.team_a_color),
            Some(BLACK),
            false,
            false,
            false,
        );
        display_controller.get_text_layer_mut().insert_string_xy(
            14,
            7,
            "🯆",
            Some(self.team_b_color),
            Some(BLACK),
            false,
            false,
            false,
        );
    }
}

/*************************************************************************************************************
**************************************************************************************************************
                                                VARIOUS FUNCTIONS
*************************************************************************************************************
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
                    if !(col_test == col && row_test == row)
                    {
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

            if current_cell.alive && (total_count < 2 || total_count > 3) {
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
                } else {
                    if a_team_count > b_team_count {
                        next_gen_cell.team = Team::A;
                    } else {
                        next_gen_cell.team = Team::B;
                    }
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

    return continue_game;
}
