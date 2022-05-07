pub struct Sprite {
    x: usize,
    y: usize,
    size_x: u8,
    size_y: u8,
    visible: bool,
    image: Vec<u8>
}

impl Sprite {

    pub fn new() -> Sprite { 

        let size: usize = 8 * 8;
        let mut new_image = Vec::new();

        // for _value in 0..size {
        //     new_image.push(0);
        // }

        Sprite {
            x: 0,
            y: 0,
            size_x: 8,
            size_y: 8,
            visible: false,
            image: new_image
        }
    }
}