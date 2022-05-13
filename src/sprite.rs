use std::env;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::*;

pub struct Sprite {
    pub pos_x: usize,
    pub pos_y: usize,
    pub size_x: usize,
    pub size_y: usize,
    pub visible: bool,
    pub h_flipp: bool,
    pub v_flipp: bool,
    pub image: Vec<u8>
}

impl Sprite {

    pub fn new() -> Sprite { 

        let size: usize = 16 * 16;
        let mut image = Vec::new();

        for i in 0..size {
            image.push(0);
        }

        Sprite {
            pos_x: 0,
            pos_y: 0,
            size_x: 16,
            size_y: 16,
            visible: true,
            h_flipp: false,
            v_flipp: false,
            image
        }
    }

    pub fn new_from_file(path_to_file: &String) -> Sprite { 

        let contents = fs::read_to_string(path_to_file).expect("Something went wrong reading the file");
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
            size_x,
            size_y,
            visible: true,
            h_flipp: false,
            v_flipp: false,
            image: data
        }
    }
}