pub struct Player {
    pub x: isize,
    pub y: isize,
    pub direction: f32
}

impl Player {
    pub fn new(x: isize, y: isize, direction: f32) -> Self {
        Player {
            x,y,direction
        }
    }
}
