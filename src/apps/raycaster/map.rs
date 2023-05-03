use std::f32::consts::PI;

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
    pub map: Vec<char>,
    pub walls: Vec<Wall>,
    pub player_start_x: isize,
    pub player_start_y: isize,
    pub player_start_dir: f32
}

impl Map {

    pub fn new() -> Self {
        Map {
            width: 9,
            height: 10,
            map: Vec::from(['▛','▀','▀','▀','▀','▀','▜','▀','▜',
                            '▌',' ',' ',' ',' ',' ','▐',' ','▐',
                            '▌',' ','█',' ','█',' ','▐',' ','▐',
                            '▌',' ',' ',' ',' ',' ','▐',' ','▐',
                            '▌',' ','█',' ','█',' ','▐',' ','▐',
                            '▌',' ',' ',' ',' ',' ','▐',' ','█',
                            '█',' ','█','█','▄','▄','▟',' ','▐',
                            '▌',' ',' ','█',' ',' ','█',' ','▐',
                            '▌',' ',' ',' ',' ',' ',' ',' ','▐',
                            '▙','▄','▄','█','▄','▄','█','▄','▟']),
            walls: Vec::new(),
            player_start_x: 7,
            player_start_y: 1,
            player_start_dir: PI/2.0 // in radians 0 is right, PI is left
        }
    }

    pub fn transform_map_into_list_of_walls(&mut self) {
        for y in 0..(self.height) {
            for x in 0..(self.width) {
                let index = x + y * self.width;
                let cell_code = self.map[index];

                //4 lines surrounding the bloc, each wall is of length SCALE
                let wall_n: Wall = Wall {
                    x1: x as isize,
                    y1: y as isize,
                    x2: x as isize + 1,
                    y2: y as isize,
                    texture: 1
                };

                let wall_e: Wall = Wall {
                    x1: x as isize + 1,
                    y1: y as isize,
                    x2: x as isize + 1,
                    y2: y as isize + 1,
                    texture: 1
                };

                let wall_s: Wall = Wall {
                    x1: x as isize + 1,
                    y1: y as isize + 1,
                    x2: x as isize,
                    y2: y as isize + 1,
                    texture: 1
                };

                let wall_w: Wall = Wall {
                    x1: x as isize,
                    y1: y as isize + 1,
                    x2: x as isize,
                    y2: y as isize,
                    texture: 1
                };

                match cell_code {
                    ' ' => (),
                    '█' => { 
                        self.walls.push(wall_n);
                        self.walls.push(wall_e);
                        self.walls.push(wall_s);
                        self.walls.push(wall_w);
                    },
                    '▛' => { self.walls.push(wall_w); self.walls.push(wall_n)},
                    '▀' => self.walls.push(wall_n),
                    '▜' => { self.walls.push(wall_n); self.walls.push(wall_e)},
                    '▐' => self.walls.push(wall_e),
                    '▟' => { self.walls.push(wall_e); self.walls.push(wall_s)},
                    '▄' => self.walls.push(wall_s),
                    '▙' => { self.walls.push(wall_w); self.walls.push(wall_s)},
                    '▌' => self.walls.push(wall_w),
                    _ => ()
                }
            }
        }
    }
}