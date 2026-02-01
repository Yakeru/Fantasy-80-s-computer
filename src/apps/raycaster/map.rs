use std::f32::consts::PI;

use fantasy_cpc_display_controller::color_palettes::{BLACK, BROWN, DARK_GREY};

use super::renderer::GAME_SCALE;

pub struct Wall {
    pub x1: isize,
    pub y1: isize,
    pub x2: isize,
    pub y2: isize,
    pub texture: u8
}

impl Wall {
    pub fn new() -> Self {
        Wall {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            texture: 0
        }
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub map: Vec<u8>,
    pub walls: Vec<Wall>,
    pub player_start_x: isize,
    pub player_start_y: isize,
    pub player_start_dir: f32,
    pub fog_distance: isize,
    pub fog_range: isize,
    pub fog_color: usize,
    pub _ground_color: usize,
    pub _sky_color: usize
}

impl Map {

    pub fn new() -> Self {
        Map {
            width: 9,
            height: 10,
            map: Vec::from([1,1,1,1,1,1,1,1,1,
                            1,0,0,0,0,0,1,0,1,
                            1,0,0,0,0,0,1,0,1,
                            1,0,0,1,0,0,1,0,1,
                            1,0,0,0,0,0,1,0,1,
                            1,0,0,0,0,0,1,0,1,
                            1,0,1,1,1,1,1,0,1,
                            1,0,0,0,0,0,0,0,1,
                            1,0,0,0,0,0,0,0,1,
                            1,1,1,1,1,1,1,1,1]),
            walls: Vec::new(),
            player_start_x: 7,
            player_start_y: 1,
            player_start_dir: PI/2.0, // in radians 0 is right, PI is left
            fog_distance: 3,
            fog_range: 2,
            fog_color: BLACK,
            _ground_color: BROWN,
            _sky_color: DARK_GREY
        }
    }

    pub fn get_cell_from_coord(&self, x: isize, y: isize) -> u8 {
        let map_x = (x as f32 / GAME_SCALE as f32).floor() as usize;
        let map_y = (y as f32 / GAME_SCALE as f32).floor() as usize;
        let index: usize = map_y * self.width + map_x;
        self.map[index]
    }

    pub fn transform_map_into_list_of_walls(&mut self) {
        for y in 0..(self.height) {
            for x in 0..(self.width) {
                let index = x + y * self.width;
                let cell_code = self.map[index];

                //4 lines surrounding the bloc, each wall is of length SCALE
                let wall_n: Wall = Wall {
                    x1: x as isize * GAME_SCALE,
                    y1: y as isize * GAME_SCALE,
                    x2: x as isize * GAME_SCALE + GAME_SCALE,
                    y2: y as isize * GAME_SCALE,
                    texture: 1
                };

                let wall_e: Wall = Wall {
                    x1: x as isize * GAME_SCALE + GAME_SCALE,
                    y1: y as isize * GAME_SCALE,
                    x2: x as isize * GAME_SCALE + GAME_SCALE,
                    y2: y as isize * GAME_SCALE + GAME_SCALE,
                    texture: 1
                };

                let wall_s: Wall = Wall {
                    x1: x as isize * GAME_SCALE + GAME_SCALE,
                    y1: y as isize * GAME_SCALE + GAME_SCALE,
                    x2: x as isize * GAME_SCALE,
                    y2: y as isize * GAME_SCALE + GAME_SCALE,
                    texture: 1
                };

                let wall_w: Wall = Wall {
                    x1: x as isize * GAME_SCALE,
                    y1: y as isize * GAME_SCALE + GAME_SCALE,
                    x2: x as isize * GAME_SCALE,
                    y2: y as isize * GAME_SCALE,
                    texture: 1
                };

                match cell_code {
                    0 => (),
                    1 => { 
                        self.walls.push(wall_n);
                        self.walls.push(wall_e);
                        self.walls.push(wall_s);
                        self.walls.push(wall_w);
                    },
                    _ => ()
                }
            }
        }
    }
}