use std::fs;

pub struct Sprite {
    pub id: String,
    pub pos_x: isize,
    pub pos_y: isize,
    pub size: SpriteSize,
    pub visible: bool,
    pub h_flipp: bool,
    pub v_flipp: bool,
    pub image: Vec<usize>,
}

pub enum SpriteSize {
    _8x8,
    _16x16,
    _32x32
}

impl SpriteSize {
    pub fn size(&self) -> (usize, usize) {
        match *self {
            SpriteSize::_8x8 => (8, 8),
            SpriteSize::_16x16 => (16, 16),
            SpriteSize::_32x32 => (32, 32),
        }
    }
}

impl Sprite {
    pub fn new(id: String) -> Sprite {
        let size: usize = 16 * 16;
        let mut image = Vec::new();

        for _i in 0..size {
            image.push(0);
        }

        Sprite {
            pos_x: 0,
            pos_y: 0,
            size: SpriteSize::_8x8,
            visible: true,
            h_flipp: false,
            v_flipp: false,
            image,
            id,
        }
    }

    pub fn new_from_file(id: String, path_to_file: &String) -> Sprite {
        let contents =
            fs::read_to_string(path_to_file).expect("Something went wrong reading the file");
        let split_text: Vec<&str> = contents.split(',').collect();
        //let size_x = split_text[0].parse::<usize>().unwrap();
        //let size_y = split_text[1].parse::<usize>().unwrap();

        let mut data: Vec<usize> = Vec::new();

        for i in 2..split_text.len() {
            data.push(split_text[i].parse::<usize>().unwrap());
        }

        Sprite {
            pos_x: 0,
            pos_y: 0,
            size: SpriteSize::_8x8,
            visible: true,
            h_flipp: false,
            v_flipp: false,
            image: data,
            id,
        }
    }
}
