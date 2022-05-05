struct Position {
    x: u32, 
    y: u32
}

struct Size {
    width: u8,
    height: u8
}

struct Sprite {
    mut Position: position,
    mut Size: size,
    mut [u8] data
}

impl Sprite {

    fn new(w: u8, h: u8) -> Sprite {

        let position = Position {
            x: 0,
            y: 0
        } 

        let size = Size {
            width: width,
            height: h
        }

        Sprite {
            position: position,
            size: size,
            data:  
        }
    }
}