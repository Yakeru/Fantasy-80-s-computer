use std::fs;

pub struct Sprite {
    pub id: String,
    pub pos_x: usize,
    pub pos_y: usize,
    pub size: SpriteSize,
    pub visible: bool,
    pub h_flipp: bool,
    pub v_flipp: bool,
    pub image: Vec<u8>,
}

pub enum SpriteSize {
    _8x8,
    _8x16,
    _16x8,
    _16x16,
    _16x32,
    _32x16,
    _32x32,
}

pub fn value_from_physical_size(x: usize, y: usize) -> SpriteSize {
    match (x, y) {
        (8, 8) => SpriteSize::_8x8,
        (8, 16) => SpriteSize::_8x16,
        (16, 8) => SpriteSize::_16x8,
        (16, 16) => SpriteSize::_16x16,
        (16, 32) => SpriteSize::_16x32,
        (32, 16) => SpriteSize::_32x16,
        (32, 32) => SpriteSize::_32x32,
        _ => SpriteSize::_16x16,
    }
}

impl Sprite {
    pub fn new(id: String) -> Sprite {
        let size: usize = 16 * 16;
        let mut image = Vec::new();

        for i in 0..size {
            image.push(0);
        }

        Sprite {
            pos_x: 0,
            pos_y: 0,
            size: SpriteSize::_16x16,
            visible: true,
            h_flipp: false,
            v_flipp: false,
            image,
            id,
        }
    }

    pub fn value_in_physical_size(&self) -> (usize, usize) {
        match self.size {
            SpriteSize::_8x8 => (8 as usize, 8 as usize),
            SpriteSize::_8x16 => (8 as usize, 16 as usize),
            SpriteSize::_16x8 => (16 as usize, 8 as usize),
            SpriteSize::_16x16 => (16 as usize, 16 as usize),
            SpriteSize::_16x32 => (16 as usize, 32 as usize),
            SpriteSize::_32x16 => (32 as usize, 16 as usize),
            SpriteSize::_32x32 => (32 as usize, 23 as usize),
        }
    }

    pub fn new_from_file(id: String, path_to_file: &String) -> Sprite {
        let contents =
            fs::read_to_string(path_to_file).expect("Something went wrong reading the file");
        let split_text: Vec<&str> = contents.split(',').collect();
        let size_x = split_text[0].parse::<usize>().unwrap();
        let size_y = split_text[1].parse::<usize>().unwrap();

        let mut data: Vec<u8> = Vec::new();

        for i in 2..split_text.len() {
            data.push(split_text[i].parse::<u8>().unwrap());
        }

        Sprite {
            pos_x: 0,
            pos_y: 0,
            size: value_from_physical_size(size_x, size_y),
            visible: true,
            h_flipp: false,
            v_flipp: false,
            image: data,
            id,
        }
    }
}
