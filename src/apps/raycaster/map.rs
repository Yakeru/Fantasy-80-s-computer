const MAP_SCALE: isize = 100;

pub struct Wall {
    pub x1: isize,
    pub y1: isize,
    pub x2: isize,
    pub y2: isize,
    pub texture: u8
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
            width: 6,
            height: 6,
            map: Vec::from(['▛', '▀', '▀', '▀', '▀', '▜',
                            '▌', ' ', ' ', ' ', ' ', '▐',
                            '▌', ' ', '█', '▜', ' ', '▐',
                            '▌', ' ', ' ', '█', ' ', '▐',
                            '▌', ' ', ' ', ' ', ' ', '▐',
                            '▙', '▄', '▄', '▄', '▄', '▟']),
            walls: Vec::new(),
            player_start_x: 150,
            player_start_y: 450,
            player_start_dir: 0.0
        }
    }

    pub fn transform_map_into_list_of_walls(&mut self) {
        for y in 0..(self.width) {
            for x in 0..(self.height) {
                let index = x + y * self.width;
                let cell_code = self.map[index];

                //4 lines surrounding the bloc, each wall is of length MAP_SCALE
                let wall_N: Wall = Wall {
                    x1: x as isize * MAP_SCALE,
                    y1: y as isize * MAP_SCALE,
                    x2: x as isize * MAP_SCALE + MAP_SCALE,
                    y2: y as isize * MAP_SCALE,
                    texture: 1
                };

                let wall_E: Wall = Wall {
                    x1: x as isize * MAP_SCALE + MAP_SCALE,
                    y1: y as isize * MAP_SCALE,
                    x2: x as isize * MAP_SCALE + MAP_SCALE,
                    y2: y as isize * MAP_SCALE + MAP_SCALE,
                    texture: 1
                };

                let wall_S: Wall = Wall {
                    x1: x as isize * MAP_SCALE + MAP_SCALE,
                    y1: y as isize * MAP_SCALE + MAP_SCALE,
                    x2: x as isize * MAP_SCALE,
                    y2: y as isize * MAP_SCALE + MAP_SCALE,
                    texture: 1
                };

                let wall_W: Wall = Wall {
                    x1: x as isize * MAP_SCALE,
                    y1: y as isize * MAP_SCALE + MAP_SCALE,
                    x2: x as isize * MAP_SCALE,
                    y2: y as isize * MAP_SCALE,
                    texture: 1
                };

                match cell_code {
                    ' ' => (),
                    '█' => { 
                        self.walls.push(wall_N);
                        self.walls.push(wall_E);
                        self.walls.push(wall_S);
                        self.walls.push(wall_W);
                    },
                    '▛' => { self.walls.push(wall_W); self.walls.push(wall_N)},
                    '▀' => self.walls.push(wall_N),
                    '▜' => { self.walls.push(wall_N); self.walls.push(wall_E)},
                    '▐' => self.walls.push(wall_E),
                    '▟' => { self.walls.push(wall_E); self.walls.push(wall_S)},
                    '▄' => self.walls.push(wall_S),
                    '▙' => { self.walls.push(wall_W); self.walls.push(wall_S)},
                    '▌' => self.walls.push(wall_W),
                    _ => ()
                }
            }
        }
    }
}