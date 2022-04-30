struct Sprite {
    struct position {
        x: u32, 
        y: u32
    }

    struct size {
        width: u8,
        height: u8
    }

    mut [u8;size.width*size.height] data;
}